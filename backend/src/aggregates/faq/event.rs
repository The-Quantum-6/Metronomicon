use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
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
