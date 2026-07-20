use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use strum::Display;
use uuid::Uuid;

use crate::aggregates::project_idea::difficulty::Difficulty;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Display)]
pub enum ProjectIdeaEvent {
    ProjectCreated {
        idea_id: Uuid,
        course_id: Uuid,
        title: String,
        body: String,
        difficulty: Option<Difficulty>,
    },
    ProjectUpdated {
        idea_id: Uuid,
        course_id: Uuid,
        title: Option<String>,
        body: Option<String>,
        difficulty: Option<Difficulty>,
    },
    ProjectDeleted {
        idea_id: Uuid,
        course_id: Uuid,
    },
}

impl DomainEvent for ProjectIdeaEvent {
    fn event_type(&self) -> String {
        self.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
