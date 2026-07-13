use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::shared::Officiality;

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
        course_id: Uuid,
        question: Option<String>,
        answer: Option<String>,
    },
    /// Delete FAQ entry
    ///
    /// Requires `write_text`
    Delete { faq_id: Uuid, course_id: Uuid },
    /// Set FAQ official status
    ///
    /// Requires `page_admin`
    SetOfficial {
        faq_id: Uuid,
        course_id: Uuid,
        officiality: Officiality,
    },
}

impl FaqCommand {
    pub fn id(&self) -> &Uuid {
        match self {
            FaqCommand::Create { faq_id, .. } => faq_id,
            FaqCommand::Delete { faq_id, .. } => faq_id,
            FaqCommand::Update { faq_id, .. } => faq_id,
            FaqCommand::SetOfficial { faq_id, .. } => faq_id,
        }
    }
}
