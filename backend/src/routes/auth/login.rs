use crate::auth::oidc;
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login/google", get(oidc::login_send))
        .route("/login/callback", get(oidc::login_callback))
        .route("/logout", post(oidc::logout))
}
