pub mod auth;
pub mod courses;
pub mod user;

use axum::Router;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().merge(user::router()).merge(auth::router()).merge(courses::router())
}