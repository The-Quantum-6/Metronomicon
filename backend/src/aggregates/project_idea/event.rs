use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum ProjectIdeaEvent {
    ProjectCreated {
        idea_id: Uuid,
        course_id: Uuid,
        title: String,
        body: String,
    },
    ProjectUpdated {
        idea_id: Uuid,
        title: Option<String>,
        body: Option<String>,
    },
    ProjectDeleted {
        idea_id: Uuid,
    },
}
