pub mod course_endpoint;

use axum::Router;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().merge(course_endpoint::router())
}
