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
    Delete,

    /// Update course metadata
    ///
    /// Requires `page_admin`
    UpdateMetadata {
        name: Option<String>,
        code: Option<String>,
        field: Option<String>,
        description: Option<String>,
    },

    /// Add tag
    ///
    /// Requires `page_admin`
    AddTag { tag: String },
    /// Remove tag
    ///
    /// Requires `page_admin`
    RemoveTag { tag: String },
}
