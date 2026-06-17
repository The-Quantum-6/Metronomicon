use crate::auth::oidc;
use crate::state::AppState;
use axum::{
    Router,
    routing::get,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login/feide", get(oidc::login_send))
        .route("/login/callback", get(oidc::login_callback))
}
