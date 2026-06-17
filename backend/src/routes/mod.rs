pub mod user;
pub mod auth;

use axum::Router;
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new().merge(user::router()).merge(auth::router())
}
