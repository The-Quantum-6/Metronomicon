use crate::models::{
    claims::{AccessClaim, PermsClaim},
    user::UserRole,
};
use jsonwebtoken::{EncodingKey, Header, encode, get_current_timestamp};
use uuid::Uuid;

pub fn generate_access(
    key: &EncodingKey,
    role: UserRole,
    sub: String,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = get_current_timestamp();

    let claims = AccessClaim {
        sub,
        role,
        iat: now,
        exp: now + 900,
    };

    encode(&Header::default(), &claims, key)
}

pub fn generate_perms(
    key: &EncodingKey,
    resource_id: String,
    perms: i32,
    sub: String,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = get_current_timestamp();

    let claims = PermsClaim {
        sub,
        resource_id,
        perms,
        iat: now,
        exp: now + 180,
    };

    encode(&Header::default(), &claims, key)
}
