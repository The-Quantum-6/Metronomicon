use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceCommand {
    /// Create resource
    ///
    /// Requires `write_file`
    Create {
        resource_id: Uuid,
        course_id: Uuid,
        title: String,
        key: String,
    },
    /// Update resource
    ///
    /// Requires `write_file`
    Update {
        resource_id: Uuid,
        course_id: Uuid,
        title: Option<String>,
        key: Option<String>,
    },
    ///Delete resource
    ///
    /// Requires `write_file`
    Delete { resource_id: Uuid, course_id: Uuid },
    /// Set resource official status
    ///
    /// Requires `page_admin`
    SetOfficial {
        resource_id: Uuid,
        course_id: Uuid,
        official: bool,
    },
}

impl ResourceCommand {
    pub fn id(&self) -> &Uuid {
        match self {
            ResourceCommand::Create { resource_id, .. } => resource_id,
            ResourceCommand::Delete { resource_id, .. } => resource_id,
            ResourceCommand::Update { resource_id, .. } => resource_id,
            ResourceCommand::SetOfficial { resource_id, .. } => resource_id,
        }
    }
}
