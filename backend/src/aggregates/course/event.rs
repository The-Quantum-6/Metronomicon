use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Display)]
pub enum CourseEvent {
    CourseCreated {
        name: String,
        code: String,
        field: String,
        description: String,
    },
    CourseDeleted,
    CourseMetadataUpdated {
        name: Option<String>,
        code: Option<String>,
        field: Option<String>,
        description: Option<String>,
    },
    TagAdded {
        tag: String,
    },
    TagRemoved {
        tag: String,
    },
}

impl DomainEvent for CourseEvent {
    fn event_type(&self) -> String {
        self.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
