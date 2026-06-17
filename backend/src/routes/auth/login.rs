use axum::{Router, extract::{Query, State}, response::Redirect, routing::get};
use openidconnect::{
    AuthorizationCode, CsrfToken, Nonce, PkceCodeChallenge, PkceCodeVerifier, Scope, OAuth2TokenResponse,
};
use openidconnect::core::{CoreAuthenticationFlow, CoreClient};
use openidconnect::reqwest;
use serde::Deserialize;
use sqlx::PgPool;
use tower_sessions::Session;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/login/feide", get(login_send))
        .route("/login/callback", get(login_callback))
}


async fn login_send() -> Redirect {

}

async fn login_callback(Query(params): Query<CallbackCode>) -> Redirect {

}

