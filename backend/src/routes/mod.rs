pub mod auth;
pub mod contribution;
pub mod course;
pub mod faq;
pub mod files;
pub mod link;
pub mod project_idea;
pub mod report;
pub mod resources;
pub mod user;
use crate::{middleware::perms::perm_middleware, state::AppState};
use axum::{Router, middleware};

pub fn protected_router(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(user::router(state.clone()))
        .merge(faq::router(state.clone()))
        .merge(report::router(state.clone()))
        .merge(link::router(state.clone()))
        .merge(project_idea::router(state.clone()))
        .merge(contribution::router(state.clone()))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(course::router())
        .merge(resources::router())
        .merge(auth::router())
        .merge(files::router())
}
