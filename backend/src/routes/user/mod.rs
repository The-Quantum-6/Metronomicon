
pub mod user_endpoint;

use axum::Router;
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new().merge(user_endpoint::router())
}
