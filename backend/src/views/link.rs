use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::shared::Status;

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct LinkDetailView {
    pub link_id: Uuid,
    pub status: Status,
    pub label: String,
    pub url: String,
    pub official: bool,
}
