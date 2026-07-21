use crate::middleware::jwt::{AuthUser, jwt_middleware};
use crate::models::claims::AccessClaim;
use crate::state::{self, AppState};
use axum::{Json, middleware};
use axum::{Router, routing::get};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/me", get(get_me))
}

pub async fn get_me(AuthUser(claims): AuthUser) -> Json<AccessClaim> {
    Json(claims)
}
