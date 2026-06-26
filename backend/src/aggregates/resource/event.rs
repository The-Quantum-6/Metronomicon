use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceEvent {
    ResourceCreated {
        resource_id: Uuid,
        course_id: Uuid,
        title: String,
        key: Uuid,
    },
    ResourceDeleted {
        resource_id: Uuid,
    },
    ResourceOfficialStatusChanged {
        resource_id: Uuid,
        official: bool,
    },
}
