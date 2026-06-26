use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

impl DomainEvent for LinkEvent {
    fn event_type(&self) -> String {
        match self {
            LinkEvent::LinkCreated { .. } => "LinkCreated",
            LinkEvent::LinkUpdated { .. } => "LinkUpdated",
            LinkEvent::LinkDeleted { .. } => "LinkDeleted",
            LinkEvent::LinkOfficialStatusChanged { .. } => "LinkOfficialStatusChanged",
        }
        .to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
