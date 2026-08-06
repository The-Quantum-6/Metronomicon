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
use crate::{middleware::{jwt::jwt_middleware, perms::perm_middleware}, state::AppState};
use axum::{Router, middleware};

pub fn protected_router(state: AppState) -> Router<AppState> {
    let perm_gated = Router::new()
        .merge(user::router(state.clone()))
        .merge(faq::router(state.clone()))
        .merge(report::router(state.clone()))
        .merge(link::router(state.clone()))
        .merge(project_idea::router(state.clone()))
        .merge(contribution::router(state.clone()))
        .merge(resources::router())
        .merge(course::protected_router(state.clone()))
        .route_layer(middleware::from_fn_with_state(state.clone(), perm_middleware))
        .route_layer(middleware::from_fn_with_state(state.clone(), jwt_middleware));

    // `files::protected_router` carries its own `jwt_middleware` layer and
    // deliberately skips `perm_middleware` (which can't parse multipart
    // bodies) — merged after the layers above so it isn't wrapped by them.
    perm_gated.merge(files::protected_router(state.clone()))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(auth::router())
        .merge(files::router())
        .merge(course::router())
}