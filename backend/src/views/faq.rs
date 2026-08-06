use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::shared::Status;

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct FaqDetailView {
    pub status: Status,
    pub official: bool,
    pub faq_id: Uuid,
    pub question: String,
    pub answer: String,
}
