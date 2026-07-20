use crate::state::{self, AppState};
use axum::{Router, routing::get};
use axum::{Json, middleware};
use crate::middleware::jwt::{AuthUser, jwt_middleware};
use crate::models::claims::AccessClaim;


pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/me", get(get_me))
        .layer(middleware::from_fn_with_state(state.clone(), jwt_middleware))
}

pub async fn get_me(AuthUser(claims): AuthUser) -> Json<AccessClaim> {
    Json(claims)
}