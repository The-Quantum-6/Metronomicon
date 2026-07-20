use async_trait::async_trait;
use cqrs_es::persist::ViewRepository;
use cqrs_es::{EventEnvelope, Query};
use std::sync::Arc;
use uuid::Uuid;

use crate::aggregates::shared::Status;
use crate::views::project_idea::ProjectIdeaDetailView;
use crate::{
    aggregates::project_idea::{aggregate::ProjectIdea, event::ProjectIdeaEvent},
    views::course::active_detailed::CourseDetailViewRepo,
};

pub struct CourseProjectIdeaQuery {
    view_repo: Arc<CourseDetailViewRepo>,
}

impl CourseProjectIdeaQuery {
    pub fn new(view_repo: Arc<CourseDetailViewRepo>) -> Self {
        Self { view_repo }
    }
}

#[async_trait]
impl Query<ProjectIdea> for CourseProjectIdeaQuery {
    async fn dispatch(&self, idea_id: &str, events: &[EventEnvelope<ProjectIdea>]) {
        for event in events {
            let course_id = match &event.payload {
                ProjectIdeaEvent::ProjectCreated { course_id, .. } => course_id.to_string(),
                ProjectIdeaEvent::ProjectUpdated { course_id, .. } => course_id.to_string(),
                ProjectIdeaEvent::ProjectDeleted { course_id, .. } => course_id.to_string(),
            };

            let (mut view, context) = match self.view_repo.load_with_context(&course_id).await {
                Ok(Some(vc)) => vc,
                Ok(None) => {
                    println!("CourseProjectIdeaQuery: no course view found for {course_id}");
                    continue;
                }
                Err(e) => {
                    println!("CourseProjectIdeaQuery load error: {e}");
                    continue;
                }
            };

            let idea_id = Uuid::parse_str(idea_id).unwrap();

            match &event.payload {
                ProjectIdeaEvent::ProjectCreated {
                    title,
                    body,
                    difficulty,
                    ..
                } => {
                    view.project_ideas.push(ProjectIdeaDetailView {
                        idea_id,
                        status: Status::Active,
                        title: title.clone(),
                        body: body.clone(),
                        difficulty: difficulty.clone(),
                    });
                }
                ProjectIdeaEvent::ProjectUpdated {
                    title,
                    body,
                    difficulty,
                    ..
                } => {
                    let p = view
                        .project_ideas
                        .iter_mut()
                        .find(|p| p.idea_id == idea_id)
                        .unwrap();
                    if let Some(title) = title {
                        p.title = title.clone();
                    }
                    if let Some(body) = body {
                        p.body = body.clone();
                    }
                    if let Some(difficulty) = difficulty {
                        p.difficulty = Some(difficulty.clone());
                    }
                }
                ProjectIdeaEvent::ProjectDeleted { .. } => {
                    view.project_ideas.retain(|p| p.idea_id != idea_id);
                }
            }

            // Save back under the course_id key
            if let Err(e) = self.view_repo.update_view(view, context).await {
                println!("CourseProjectIdeaQuery save error: {e}");
            }
        }
    }
}
