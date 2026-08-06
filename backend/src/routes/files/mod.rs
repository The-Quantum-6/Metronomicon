pub mod files_endpoint;

use crate::state::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new().merge(files_endpoint::router())
}

pub fn protected_router(state: AppState) -> Router<AppState> {
    Router::new().merge(files_endpoint::protected_router(state))
}
