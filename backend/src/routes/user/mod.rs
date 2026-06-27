pub mod user_endpoint;

use crate::state::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new().merge(user_endpoint::router())
}
