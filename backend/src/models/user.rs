use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
    Root
}

#[derive(Serialize, Deserialize, sqlx::Type)]
pub struct User {
    pub id: Uuid,
    pub sub: Uuid,
    pub name: String,
    pub role: UserRole,
    pub email: Option<String>,
}