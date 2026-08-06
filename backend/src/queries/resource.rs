use async_trait::async_trait;
use cqrs_es::persist::ViewRepository;
use cqrs_es::{EventEnvelope, Query};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use uuid::Uuid;

use crate::aggregates::shared::{Officiality, Status};
use crate::views::resource::ResourceDetailView;
use crate::{
    aggregates::resource::{aggregate::Resource, event::ResourceEvent},
    views::course::active_detailed::CourseDetailViewRepo,
};

/// Whether `key` belongs to a resource that is Active in an Active course —
/// i.e. an approved, undeleted resource. Used to gate file downloads so
/// pending (not-yet-approved) uploads aren't publicly reachable.
pub async fn resource_key_is_active(pool: &Pool<Postgres>, key: &str) -> Result<bool, sqlx::Error> {
    let exists: Option<bool> = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1
            FROM course_detail_view,
                 jsonb_array_elements(payload->'resources') AS resource
            WHERE payload->>'status' = 'Active'
              AND resource->>'key' = $1
              AND resource->>'status' = 'Active'
        )",
    )
    .bind(key)
    .fetch_one(pool)
    .await?;

    Ok(exists.unwrap_or(false))
}

pub struct CourseResourceQuery {
    view_repo: Arc<CourseDetailViewRepo>,
}

impl CourseResourceQuery {
    pub fn new(view_repo: Arc<CourseDetailViewRepo>) -> Self {
        Self { view_repo }
    }
}

#[async_trait]
impl Query<Resource> for CourseResourceQuery {
    async fn dispatch(&self, resource_id: &str, events: &[EventEnvelope<Resource>]) {
        for event in events {
            let course_id = match &event.payload {
                ResourceEvent::ResourceCreated { course_id, .. } => course_id.to_string(),
                ResourceEvent::ResourceUpdated { course_id, .. } => course_id.to_string(),
                ResourceEvent::ResourceDeleted { course_id, .. } => course_id.to_string(),
                ResourceEvent::ResourceOfficialStatusChanged { course_id, .. } => {
                    course_id.to_string()
                }
            };

            let (mut view, context) = match self.view_repo.load_with_context(&course_id).await {
                Ok(Some(vc)) => vc,
                Ok(None) => {
                    println!("CourseResourceQuery: no course view found for {course_id}");
                    continue;
                }
                Err(e) => {
                    println!("CourseResourceQuery load error: {e}");
                    continue;
                }
            };

            let resource_id = Uuid::parse_str(resource_id).unwrap();

            match &event.payload {
                ResourceEvent::ResourceCreated {
                    course_id,
                    title,
                    key,
                    ..
                } => {
                    view.resources.push(ResourceDetailView {
                        resource_id,
                        status: Status::Active,
                        officiality: Officiality::Unofficial,
                        course_id: *course_id,
                        title: title.clone(),
                        key: key.clone(),
                    });
                }
                ResourceEvent::ResourceUpdated { title, key, .. } => {
                    let r = view
                        .resources
                        .iter_mut()
                        .find(|r| r.resource_id == resource_id)
                        .unwrap();
                    if let Some(title) = title {
                        r.title = title.clone();
                    }
                    if let Some(key) = key {
                        r.key = key.clone();
                    }
                }
                ResourceEvent::ResourceDeleted { .. } => {
                    view.resources.retain(|r| r.resource_id != resource_id);
                }
                ResourceEvent::ResourceOfficialStatusChanged { officiality, .. } => {
                    let r = view
                        .resources
                        .iter_mut()
                        .find(|r| r.resource_id == resource_id)
                        .unwrap();
                    r.officiality = officiality.clone();
                }
            }

            if let Err(e) = self.view_repo.update_view(view, context).await {
                println!("CourseResourceQuery save error: {e}");
            }
        }
    }
}
