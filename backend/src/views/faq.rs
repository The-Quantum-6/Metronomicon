use serde::{Deserialize, Serialize};
use uuid::Uuid;


use crate::aggregates::shared::{Officiality, Status};

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct FaqDetailView {
    pub status: Status,
    pub officiality: Officiality,
    pub faq_id: Uuid,
    pub course_id: Uuid,
    pub question: String,
    pub answer: String,
}


