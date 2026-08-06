use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::{project_idea::difficulty::Difficulty, shared::Status};

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct ProjectIdeaDetailView {
    pub idea_id: Uuid,
    pub status: Status,
    pub title: String,
    pub body: String,
    pub difficulty: Option<Difficulty>,
    pub official: bool,
}
