use async_trait::async_trait;
use cqrs_es::persist::ViewRepository;
use cqrs_es::{EventEnvelope, Query};
use std::sync::Arc;
use uuid::Uuid;

use crate::aggregates::shared::Status;
use crate::views::link::LinkDetailView;
use crate::{
    aggregates::link::{aggregate::Link, event::LinkEvent},
    views::course::CourseDetailViewRepo,
};

pub struct CourseLinkQuery {
    view_repo: Arc<CourseDetailViewRepo>,
}

impl CourseLinkQuery {
    pub fn new(view_repo: Arc<CourseDetailViewRepo>) -> Self {
        Self { view_repo }
    }
}

#[async_trait]
impl Query<Link> for CourseLinkQuery {
    async fn dispatch(&self, link_id: &str, events: &[EventEnvelope<Link>]) {
        for event in events {
            let course_id = match &event.payload {
                LinkEvent::LinkCreated { course_id, .. } => course_id.to_string(),
                LinkEvent::LinkUpdated { course_id, .. } => course_id.to_string(),
                LinkEvent::LinkDeleted { course_id, .. } => course_id.to_string(),
                LinkEvent::LinkOfficialStatusChanged { course_id, .. } => course_id.to_string(),
            };

            let (mut view, context) = match self.view_repo.load_with_context(&course_id).await {
                Ok(Some(vc)) => vc,
                Ok(None) => {
                    println!("CourseLinkQuery: no course view found for {course_id}");
                    continue;
                }
                Err(e) => {
                    println!("CourseLinkQuery load error: {e}");
                    continue;
                }
            };

            let link_id = Uuid::parse_str(link_id).unwrap();

            match &event.payload {
                LinkEvent::LinkCreated { url, label, .. } => {
                    view.links.push(LinkDetailView {
                        link_id,
                        status: Status::Active,
                        url: url.clone(),
                        label: label.clone(),
                        official: false,
                    });
                }
                LinkEvent::LinkUpdated { label, url, .. } => {
                    let l = view
                        .links
                        .iter_mut()
                        .find(|l| l.link_id == link_id)
                        .unwrap();
                    if let Some(label) = label {
                        l.label = label.clone();
                    }
                    if let Some(url) = url {
                        l.url = url.clone();
                    }
                }
                LinkEvent::LinkDeleted { .. } => {
                    view.links.retain(|l| l.link_id != link_id);
                }
                LinkEvent::LinkOfficialStatusChanged { official, .. } => {
                    let l = view
                        .links
                        .iter_mut()
                        .find(|l| l.link_id == link_id)
                        .unwrap();
                    l.official = *official;
                }
            }

            // Save back under the course_id key
            if let Err(e) = self.view_repo.update_view(view, context).await {
                println!("CourseLinkQuery save error: {e}");
            }
        }
    }
}
