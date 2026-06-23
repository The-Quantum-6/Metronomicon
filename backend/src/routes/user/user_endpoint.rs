use axum::{Router, extract::Path, routing::get};
use sqlx::PgPool;
use crate::middleware::perms::has_perm;


use crate::models::permissions::{self, Permissions};
use bitflags::bitflags;


pub fn router() -> Router<PgPool> {
    Router::new()
    .route("/testuser", get(user_test))
    .route("/checkperms/{permissions}", get(check_permissions))
}

async fn user_test() -> String {
    "User test endpoint".to_string()
}

async fn check_permissions(Path(permissions): Path<u32>) -> String {
    let raw_perms_from_db: Permissions = Permissions::from_bits_retain(permissions);
    let har_tilgang = has_perm(raw_perms_from_db, Permissions::READ | Permissions::WRITE_TEXT | Permissions::SUGGEST_TEXT);
    har_tilgang.to_string()
}