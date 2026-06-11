use axum::{Router, routing::get};

use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/login", get(login_send))
        .route("/login/callback", get(login_callback))
        .route("/user", get(user_info))
}

async fn login_send() -> &'static str {
    let response_type = "code";
    let client_id = std::env::var("FEIDE_CLIENT_ID").expect("FEIDE_CLIENT_ID must be set");
    let redirect_uri="http://localhost:3000/login/callback";
    let scope="openid";
    let state="whatever";
}
async fn login_callback() -> &'static str { "handle login callback" }
async fn user_info() -> &'static str { "fetch user info" }