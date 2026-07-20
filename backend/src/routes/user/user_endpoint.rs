use crate::handlers::perms::{add_permission, can_transfer, has_permission, remove_permission};
use crate::middleware::jwt::AuthUser;
use crate::middleware::jwt::{self, jwt_middleware};
use crate::models::permissions::{self, Permissions};
use crate::state::AppState;
use axum::response::IntoResponse;
use axum::{Router, extract::Path, middleware, routing::get, Json};
use bitflags::bitflags;
use sqlx::PgPool;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/testuser", get(user_test))
        .route("/checkperms/{permissions}", get(check_permissions))
        .route("/me", get(me))
        .layer(middleware::from_fn_with_state(state, jwt_middleware))
}

async fn user_test() -> String {
    "User test endpoint".to_string()
}

async fn check_permissions(Path(permissions): Path<i32>) -> String {
    let raw_perms_from_db: Permissions = Permissions::from_bits_retain(permissions);
    let har_tilgang = has_permission(
        raw_perms_from_db,
        Permissions::READ | Permissions::WRITE_TEXT | Permissions::SUGGEST_TEXT,
    );
    har_tilgang.to_string()
}

async fn transfer_permissions(Path(permissions): Path<i32>) -> String {
    let raw_perms_from_db: Permissions = Permissions::from_bits_retain(permissions);
    let can_transfer_perms = can_transfer(raw_perms_from_db, Permissions::WRITE_TEXT);
    can_transfer_perms.to_string()
}

pub async fn me(AuthUser(claims): AuthUser) -> impl IntoResponse {
    Json(claims)
}
