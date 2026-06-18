use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new().route("/testuser", get(user_test))
}

async fn user_test() -> String {
    "User test endpoint".to_string()
}
