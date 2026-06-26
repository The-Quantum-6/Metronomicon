use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum FaqEvent {
    FaqCreated {
        faq_id: Uuid,
        course_id: Uuid,
        question: String,
        answer: String,
    },
    FaqUpdated {
        faq_id: Uuid,
        question: Option<String>,
        answer: Option<String>,
    },
    FaqDeleted {
        faq_id: Uuid,
    },
}

impl DomainEvent for FaqEvent {
    fn event_type(&self) -> String {
        match self {
            FaqEvent::FaqCreated { .. } => "FaqCreated",
            FaqEvent::FaqUpdated { .. } => "FaqUpdated",
            FaqEvent::FaqDeleted { .. } => "FaqDeleted",
        }
        .to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
