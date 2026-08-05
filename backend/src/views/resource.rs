use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::shared::Status;

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct ResourceDetailView {
    pub status: Status,
    pub official: bool,
    pub resource_id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub key: String,
}
