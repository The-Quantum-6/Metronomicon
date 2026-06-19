pub mod course_endpoint;

use axum::Router;
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new().merge(course_endpoint::router())
}
