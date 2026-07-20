pub mod auth;
pub mod courses;
pub mod files;
pub mod user;

use crate::state::AppState;
use axum::Router;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(user::router(state))
        .merge(courses::router())
        .merge(auth::router())
        .merge(files::router())
}
