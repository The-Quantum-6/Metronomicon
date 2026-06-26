use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::suggestion::command::Suggestion;

#[derive(Debug, Serialize, Deserialize)]
pub enum SuggestionEvent {
    SuggestionProposed {
        suggestion_id: Uuid,
        course_id: Uuid,
        kind: Suggestion,
        payload: serde_json::Value,
        proposer: Uuid,
    },
    SuggestionApproved {
        suggestion_id: Uuid,
    },
    SuggestionDenied {
        suggestion_id: Uuid,
    },
}
