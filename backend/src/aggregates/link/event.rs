use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use strum::Display;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Display)]
pub enum LinkEvent {
    LinkCreated {
        link_id: Uuid,
        course_id: Uuid,
        label: String,
        url: String,
    },
    LinkUpdated {
        link_id: Uuid,
        course_id: Uuid,
        label: Option<String>,
        url: Option<String>,
    },
    LinkDeleted {
        link_id: Uuid,
        course_id: Uuid,
    },
    LinkOfficialStatusChanged {
        link_id: Uuid,
        course_id: Uuid,
        official: bool,
    },
}

impl DomainEvent for LinkEvent {
    fn event_type(&self) -> String {
        self.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
