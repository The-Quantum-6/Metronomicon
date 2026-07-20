use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::shared::{Officiality, Status};

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct ResourceDetailView {
    pub status: Status,
    pub officiality: Officiality,
    pub resource_id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub key: String,
}
