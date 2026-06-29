pub mod courses;
pub mod user;
pub mod files;

use axum::Router;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().merge(user::router()).merge(courses::router()).merge(files::router())
}
