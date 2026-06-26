use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum FaqCommand {
    /// Create FAQ entry
    ///
    /// Requires `write_text`
    Create {
        faq_id: Uuid,
        course_id: Uuid,
        question: String,
        answer: String,
    },
    /// Update FAQ entry
    ///
    /// Requires `write_text`
    Update {
        faq_id: Uuid,
        question: Option<String>,
        answer: Option<String>,
    },
    /// Delete FAQ entry
    ///
    /// Requires `write_text`
    Delete { faq_id: Uuid },
}
