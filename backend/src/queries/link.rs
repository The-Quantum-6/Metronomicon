use async_trait::async_trait;
use cqrs_es::persist::ViewRepository;
use cqrs_es::{EventEnvelope, Query};
use std::sync::Arc;

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
    async fn dispatch(&self, _link_id: &str, events: &[EventEnvelope<Link>]) {
        for event in events {
            let course_id = match &event.payload {
                LinkEvent::LinkCreated { course_id, .. } => course_id.to_string(),
                _ => todo!("Only link creation is supported atm"),
            };

            let (mut view, context) = match self.view_repo.load_with_context(&course_id).await {
                Ok(Some(vc)) => vc,
                Ok(None) => {
                    panic!("CourseLinkQuery: no course view found for {course_id}");
                }
                Err(e) => {
                    panic!("CourseLinkQuery load error: {e}");
                }
            };

            match &event.payload {
                LinkEvent::LinkCreated {
                    link_id,
                    url,
                    label,
                    ..
                } => {
                    view.links.push(LinkDetailView {
                        link_id: link_id.clone(),
                        url: url.clone(),
                        label: label.clone(),
                        official: false,
                    });
                }
                _ => todo!("Only link creation is supported atm"),
            }

            // Save back under the course_id key
            if let Err(e) = self.view_repo.update_view(view, context).await {
                println!("CourseLinkQuery save error: {e}");
            }
        }
    }
}
