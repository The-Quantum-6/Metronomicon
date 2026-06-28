use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum CourseCommand {
    /// Create a new course
    ///
    /// `SUPERUSER` only
    Create {
        course_id: Uuid,
        name: String,
        code: String,
        field: String,
        description: String,
    },

    /// Delete course
    ///
    /// `SUPERUSER` only
    Delete { course_id: Uuid },

    /// Update course metadata
    ///
    /// Requires `page_admin`
    UpdateMetadata {
        course_id: Uuid,
        name: Option<String>,
        code: Option<String>,
        field: Option<String>,
        description: Option<String>,
    },

    /// Add tag
    ///
    /// Requires `page_admin`
    AddTag { course_id: Uuid, tag: String },
    /// Remove tag
    ///
    /// Requires `page_admin`
    RemoveTag { course_id: Uuid, tag: String },
}

impl CourseCommand {
    pub fn id(&self) -> &Uuid {
        match self {
            CourseCommand::Create { course_id, .. } => course_id,
            CourseCommand::Delete { course_id, .. } => course_id,
            CourseCommand::UpdateMetadata { course_id, .. } => course_id,
            CourseCommand::AddTag { course_id, .. } => course_id,
            CourseCommand::RemoveTag { course_id, .. } => course_id,
        }
    }
}
