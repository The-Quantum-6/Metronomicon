use async_trait::async_trait;
use cqrs_es::{EventEnvelope, Query};
use cqrs_es::persist::{ViewContext, ViewRepository};
use std::sync::Arc;

use crate::views::link::LinkDetailView;
use crate::{
    aggregates::link::{aggregate::Link, event::LinkEvent},
    views::course::{CourseDetailView, CourseDetailViewRepo},
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
            // Extract the course_id from the link event — this is the key.
            // Your link events need to carry the course_id they belong to.
            let course_id = match &event.payload {
                LinkEvent::LinkCreated { course_id, .. } => course_id.to_string(),
                _ => todo!("Only link creation is supported atm")
            };

            // Load the course view by course_id (the correct row)
            let mut view: CourseDetailView = match self.view_repo.load(&course_id).await {
                Ok(Some(v)) => v,
                Ok(None) => {
                    panic!("CourseLinkQuery: no course view found for {course_id}");
                }
                Err(e) => {
                    panic!("CourseLinkQuery load error: {e}");
                }
            };

            // Mutate the view based on the link event
            match &event.payload {
                LinkEvent::LinkCreated { link_id, url, label, .. } => {
                    view.links.push(LinkDetailView {
                        link_id: link_id.clone(),
                        url: url.clone(),
                        label: label.clone(),
                        official: false,
                    });
                },
                _ => todo!("Only link creation is supported atm")
            }

            // Save back under the course_id key
            if let Err(e) = self.view_repo.update_view(view, ViewContext::new(course_id.to_string(), 1)).await {
                println!("CourseLinkQuery save error: {e}");
            }
        }
    }
}