use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum LinkEvent {
    LinkCreated {
        link_id: Uuid,
        course_id: Uuid,
        label: String,
        url: String,
    },
    LinkUpdated {
        link_id: Uuid,
        label: Option<String>,
        url: Option<String>,
    },
    LinkDeleted {
        link_id: Uuid,
    },
    LinkOfficialStatusChanged {
        link_id: Uuid,
        official: bool,
    },
}
