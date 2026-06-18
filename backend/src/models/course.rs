use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::Type)]
pub struct Course {
    pub id: Uuid,
    pub content: Option<String>,
    pub name: String,
    pub code: String,
}