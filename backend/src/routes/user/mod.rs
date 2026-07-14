pub mod user_endpoint;

use axum::Router;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().merge(user_endpoint::router())
}
