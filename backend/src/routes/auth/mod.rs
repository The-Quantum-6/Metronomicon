pub mod oidc;
pub mod login;

use axum::Router;
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new().merge(login::router())
}
