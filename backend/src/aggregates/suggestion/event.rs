use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::suggestion::command::Suggestion;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum SuggestionEvent {
    SuggestionProposed {
        suggestion_id: Uuid,
        course_id: Uuid,
        kind: Suggestion,
    },
    SuggestionApproved {
        suggestion_id: Uuid,
    },
    SuggestionDenied {
        suggestion_id: Uuid,
    },
}

impl DomainEvent for SuggestionEvent {
    fn event_type(&self) -> String {
        match self {
            SuggestionEvent::SuggestionProposed { .. } => "SuggestionProposed",
            SuggestionEvent::SuggestionApproved { .. } => "SuggestionApproved",
            SuggestionEvent::SuggestionDenied { .. } => "SuggestionDenied",
        }
        .to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
