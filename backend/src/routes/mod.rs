pub mod course;
pub mod faq;
pub mod files;
pub mod link;
pub mod project_idea;
pub mod user;

use axum::Router;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(user::router())
        .merge(course::router())
        .merge(link::router())
        .merge(project_idea::router())
        .merge(faq::router())
}
