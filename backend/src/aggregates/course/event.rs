use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum CourseEvent {
    CourseCreated {
        id: Uuid,
        name: String,
        code: String,
        field: String,
        description: String,
    },
    CourseDeleted {
        id: Uuid,
    },
    CourseMetadataUpdated {
        id: Uuid,
        name: Option<String>,
        code: Option<String>,
        field: Option<String>,
        description: Option<String>,
    },
    TagAdded {
        id: Uuid,
        tag: String,
    },
    TagRemoved {
        id: Uuid,
        tag: String,
    },
}
