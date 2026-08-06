use crate::middleware::jwt::AuthUser;
use crate::models::user::User;
use crate::repositories::user::get_user_by_id;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, Router, routing::get};
use uuid::Uuid;

pub fn router(_state: AppState) -> Router<AppState> {
    Router::new().route("/me", get(get_me))
}

pub async fn get_me(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
) -> Result<Json<User>, StatusCode> {
    let id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::BAD_REQUEST)?;

    let user = get_user_by_id(&state.pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}
