use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

impl DomainEvent for CourseEvent {
    fn event_type(&self) -> String {
        match self {
            CourseEvent::CourseCreated { .. } => "CourseCreated",
            CourseEvent::CourseDeleted { .. } => "CourseDeleted",
            CourseEvent::CourseMetadataUpdated { .. } => "CourseMetadataUpdated",
            CourseEvent::TagAdded { .. } => "TagAdded",
            Self::TagRemoved { .. } => "TagRemoved",
        }.to_string()
    }
    
    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
