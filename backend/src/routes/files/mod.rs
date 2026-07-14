pub mod files_endpoint;

use crate::state::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new().merge(files_endpoint::router())
}
