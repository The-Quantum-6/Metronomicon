use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum CourseCommand {
    /// Create a new course
    ///
    /// `SUPERUSER` only
    Create {
        id: Uuid,
        name: String,
        code: String,
        field: String,
        description: String,
    },

    /// Delete course
    ///
    /// `SUPERUSER` only
    Delete { id: Uuid },

    /// Update course metadata
    ///
    /// Requires `page_admin`
    UpdateMetadata {
        id: Uuid,
        name: Option<String>,
        code: Option<String>,
        field: Option<String>,
        description: Option<String>,
    },

    /// Add tag
    ///
    /// Requires `page_admin`
    AddTag { id: Uuid, tag: String },
    /// Remove tag
    ///
    /// Requires `page_admin`
    RemoveTag { id: Uuid, tag: String },
}

impl CourseCommand {
    pub fn id(&self) -> &Uuid {
        match self {
            CourseCommand::Create { id, .. } => id,
            CourseCommand::Delete { id, .. } => id,
            CourseCommand::UpdateMetadata { id, .. } => id,
            CourseCommand::AddTag { id, .. } => id,
            CourseCommand::RemoveTag { id, .. } => id,
        }
    }
}
