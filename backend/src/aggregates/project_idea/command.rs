use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum ProjectIdeaCommand {
    /// Create project idea
    ///
    /// Requires `write_text`
    Create {
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
