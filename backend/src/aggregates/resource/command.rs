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
        key: Uuid,
    },
    /// Delete resource
    ///
    /// Requires `write_file`
    Delete { resource_id: Uuid },
    /// Set resource official status
    ///
    /// Requires `page_admin`
    SetOfficial { resource_id: Uuid, official: bool },
}
