use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum ProjectIdeaCommand {
    /// Create project idea
    ///
    /// Requires `write_text`
    Create {
        idea_id: Uuid,
        course_id: Uuid,
        title: String,
        body: String,
    },
    /// Update project idea
    ///
    /// Requires `write_text`
    Update {
        idea_id: Uuid,
        title: Option<String>,
        body: Option<String>,
    },
    /// Delete project idea
    ///
    /// Required `write_text`
    Delete { idea_id: Uuid },
}

impl ProjectIdeaCommand {
    pub fn id(&self) -> &Uuid {
        match self {
            ProjectIdeaCommand::Create {
                idea_id,
                course_id,
                title,
                body
            } => idea_id,
            ProjectIdeaCommand::Update {
                idea_id,
                title,
                body,
            } => idea_id,
            ProjectIdeaCommand::Delete { idea_id } => idea_id,
        }
    }
}
