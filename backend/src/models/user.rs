use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::Type)]
pub struct User {
    pub id: Uuid,
    pub sub: Uuid,
    pub name: String,
    pub email: Option<String>,
}