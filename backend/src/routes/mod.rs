mod feide;

use axum::Router;
use sqlx::PgPool;

pub fn app_router() -> Router<PgPool> {
    Router::new()
        .merge(feide::router())
}