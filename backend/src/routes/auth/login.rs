use axum::{Router, extract::{Query, State}, response::Redirect, routing::get};
use openidconnect::{
    AuthorizationCode, CsrfToken, Nonce, PkceCodeChallenge, Scope, TokenResponse
};
use openidconnect::core::{CoreAuthenticationFlow};
use serde::Deserialize;
use tower_sessions::Session;
use crate::state::AppState;
use crate::auth::oidc;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login/feide", get(oidc::login_send))
        .route("/login/callback", get(oidc::login_callback))
}
