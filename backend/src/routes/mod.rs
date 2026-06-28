pub mod course;
pub mod link;
pub mod user;

use axum::Router;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(user::router())
        .merge(course::router())
        .merge(link::router())
}
