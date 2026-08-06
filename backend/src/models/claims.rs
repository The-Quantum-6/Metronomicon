use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::user::UserRole;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessClaim {
    pub sub: String,
    pub role: UserRole,
    pub exp: u64,
    pub iat: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermsClaim {
    pub sub: String,
    pub resource_id: String,
    pub perms: i32,
    pub role: UserRole,
    pub exp: u64,
    pub iat: u64,
}
