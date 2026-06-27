use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use strum::Display;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Display)]
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

impl DomainEvent for ResourceEvent {
    fn event_type(&self) -> String {
        self.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
