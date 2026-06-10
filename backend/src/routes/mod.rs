mod feide;

use axum::Router;

pub fn app_router() -> Router {
    Router::new()
        .merge(feide::router())
}