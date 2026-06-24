pub mod user;

use axum::Router;
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new().merge(user::router())
}
