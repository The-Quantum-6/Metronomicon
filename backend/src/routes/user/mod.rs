pub mod user_endpoint;

use crate::state::AppState;
use axum::Router;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new().merge(user_endpoint::router(state))
}
