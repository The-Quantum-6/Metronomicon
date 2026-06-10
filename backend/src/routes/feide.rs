use axum::{Router, routing::get};

use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/login", get(login_send))
        .route("/login/callback", get(login_callback))
        .route("/user", get(user_info))
}

async fn login_send() -> &'static str { "send login request" }
async fn login_callback() -> &'static str { "handle login callback" }
async fn user_info() -> &'static str { "fetch user info" }