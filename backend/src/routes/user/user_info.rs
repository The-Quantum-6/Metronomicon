use axum::{extract::State, http::StatusCode, routing::get, Json, Router};


use crate::{models::user::User, repositories::user::get_users, state::AppState};
pub fn router() -> Router<AppState> {
    Router::new().route("/user", get(user_info))
}
async fn user_info(State(app_state): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = get_users(&app_state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(users))
}