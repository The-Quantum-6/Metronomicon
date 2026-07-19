use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessClaim {
    pub sub: String,     
    pub role: String,     
    pub exp: u64,
    pub iss: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermsClaim {
    pub sub: String,       
    pub resource_id: String,
    pub perms: i32,       
    pub exp: u64,
    pub iss: u64,
}