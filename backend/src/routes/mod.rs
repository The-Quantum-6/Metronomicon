pub mod course;
pub mod link;
pub mod user;
pub mod faq;

use axum::Router;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(user::router())
        .merge(course::router())
        .merge(link::router())
        .merge(faq::router())
}
