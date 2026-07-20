use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum LinkCommand {
    /// Create link
    ///
    /// Requires `write_text`
    Create {
        link_id: Uuid,
        course_id: Uuid,
        label: String,
        url: String,
    },
    /// Update link
    ///
    /// Requires `write_text`
    Update {
        link_id: Uuid,
        course_id: Uuid,
        label: Option<String>,
        url: Option<String>,
    },
    /// Delete link
    ///
    /// Requires `write_text`
    Delete { link_id: Uuid, course_id: Uuid },
    /// Set links official status
    ///
    /// Requires `page_admin`
    SetOfficial {
        link_id: Uuid,
        course_id: Uuid,
        official: bool,
    },
}

impl LinkCommand {
    pub fn id(&self) -> &Uuid {
        match self {
            LinkCommand::Create { link_id, .. } => link_id,
            LinkCommand::Update { link_id, .. } => link_id,
            LinkCommand::Delete { link_id, .. } => link_id,
            LinkCommand::SetOfficial { link_id, .. } => link_id,
        }
    }
}
