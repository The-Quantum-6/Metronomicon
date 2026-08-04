pub mod auth;
pub mod contribution;
pub mod course;
pub mod faq;
pub mod files;
pub mod grades;
pub mod link;
pub mod project_idea;
pub mod permissions;
pub mod report;
pub mod resources;
pub mod user;
use crate::{
    middleware::{jwt::jwt_middleware, perms::perm_middleware},
    state::AppState,
};
use aws_sdk_s3::types::Permission;
use axum::{Router, middleware};

pub fn protected_router(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(faq::router(state.clone()))
        .merge(report::router(state.clone()))
        .merge(link::router(state.clone()))
        .merge(project_idea::router(state.clone()))
        .merge(course::protected_router(state.clone()))
        .merge(contribution::post_router(state.clone()))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            perm_middleware,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            jwt_middleware,
        ))
}

pub fn authed_router(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(user::router(state.clone()))
        .merge(permissions::router(state.clone()))
        .merge(permissions::protected_router(state.clone()))
        .merge(contribution::get_router(state.clone()))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            jwt_middleware,
        ))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(resources::router())
        .merge(auth::router())
        .merge(files::router())
        .merge(course::router())
        .merge(grades::router())
}
