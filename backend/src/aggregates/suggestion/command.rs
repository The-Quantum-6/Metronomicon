use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::shared::moderation::{ModerationVerdict, SuggestionKind};

#[derive(Debug, Serialize, Deserialize)]
pub enum SuggestionCommand {
    /// Suggest a change
    ///
    /// Requires `suggest_file` for resource suggestions and `suggest_text` for other suggestions
    Propose {
        course_id: Uuid,
        kind: SuggestionKind,
        payload: serde_json::Value, // or a typed enum if you prefer type-safety
        proposer: Uuid,
    },

    /// Suggest a change
    ///
    /// Requires `moderate_file` for resource suggestions and `moderate_text` for other suggestions
    Moderate {
        suggestion_id: Uuid,
        verdict: ModerationVerdict,
    },
}
