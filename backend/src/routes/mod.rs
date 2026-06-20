pub mod user;

use axum::Router;
use sqlx::PgPool;

/// Main Router, merges domain-specific routers
pub fn router() -> Router<PgPool> {
    Router::new().merge(user::router())
}
