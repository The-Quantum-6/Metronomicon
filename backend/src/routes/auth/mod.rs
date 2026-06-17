pub mod feide;

use axum::Router;
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new().merge(feide::router())
}
