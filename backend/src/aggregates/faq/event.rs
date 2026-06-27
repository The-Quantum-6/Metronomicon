use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use strum::Display;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Display)]
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
        self.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
