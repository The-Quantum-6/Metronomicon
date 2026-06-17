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

#[derive(Deserialize)]
struct CallbackParams {
    code: String,
    state: String,
}

async fn login_send(session: Session, State(client): State<CoreClient>) -> Redirect {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let csrf_token = CsrfToken::new_random();
    let nonce = Nonce::new_random();

    session.insert("pkce_verifier", pkce_verifier).expect("Should store PKCE verifier in session");
    session.insert("csrf_token", csrf_token.clone()).expect("Should store CSRF token in session");
    session.insert("nonce", nonce.clone()).expect("Should store nonce in session");

    let auth_url = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            |params| {
                params.set_pkce_challenge(pkce_challenge);
                params.set_csrf_token(csrf_token);
                params.set_nonce(nonce);
                params.add_scope(Scope::new("openid".to_string()));
                params.add_scope(Scope::new("profile".to_string()));
                params.add_scope(Scope::new("email".to_string()));
            },
        )
        .url();

    Redirect::to(auth_url.as_str())
}

async fn login_callback(Query(params): Query<CallbackCode>) -> Redirect {

}

