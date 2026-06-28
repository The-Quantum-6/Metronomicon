use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct LinkDetailView {
    pub link_id: Uuid,
    pub label: String,
    pub url: String,
    pub official: bool,
}