pub mod course;
pub mod link;
pub mod user;
pub mod project_idea;

use axum::Router;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(user::router())
        .merge(course::router())
        .merge(link::router())
        .merge(project_idea::router())
}
