use jsonwebtoken::{EncodingKey, Header, encode, get_current_timestamp};
use uuid::Uuid;
use crate::models::claims::{AccessClaim, PermsClaim};

pub fn generate_access(
    key: &EncodingKey,
    role: String,
    sub: String,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = get_current_timestamp();

    let claims = AccessClaim {
        sub,
        role,
        iss: now,
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
        iss: now,
        exp: now + 180,
    };

    encode(&Header::default(), &claims, key)
}