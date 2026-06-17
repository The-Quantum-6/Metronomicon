use axum::{Router, extract::{Query, State}, response::Redirect, routing::get};
use openidconnect::{
    AuthorizationCode, CsrfToken, Nonce, PkceCodeChallenge, Scope, TokenResponse
};
use openidconnect::core::{CoreAuthenticationFlow};
use serde::Deserialize;
use tower_sessions::Session;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login/feide", get(login_send))
        .route("/login/callback", get(login_callback))
}

#[derive(Deserialize)]
struct CallbackParams {
    code: String,
    state: String,
}

async fn login_send(session: Session, State(client): State<AppState>) -> Result<Redirect, (axum::http::StatusCode, String)> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token, nonce) = client.oidc_client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    session.insert("csrf_token", csrf_token.secret()).await
    .map_err(|e| {
        eprintln!("Session error: {:?}", e);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to save CSRF token".into())
    })?;
    session.insert("nonce", nonce.secret()).await.map_err(|e| {
        eprintln!("Session error: {:?}", e);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to save CSRF token".into())
    })?;
    session.insert("pkce_verifier", pkce_verifier.secret()).await.map_err(|e| {
        eprintln!("Session error: {:?}", e);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to save CSRF token".into())
    })?;

    Ok(Redirect::to(auth_url.as_str()))
}

async fn login_callback(Query(params): Query<CallbackParams>, State(app_state): State<AppState>, session: Session) -> Result<Redirect, (axum::http::StatusCode, String)> {
    let stored_csrf: String = match session.get("csrf_token").await {
        Ok(Some(v)) => v,
        _ => return Err((axum::http::StatusCode::BAD_REQUEST, "Missing CSRF token".into())),
    };
    if stored_csrf != params.state {
        return Err((axum::http::StatusCode::BAD_REQUEST, "CSRF token mismatch".into()));
    }
    let pkce_verifier = match session.get("pkce_verifier").await {
        Ok(Some(v)) => v,
        _ => return Err((axum::http::StatusCode::BAD_REQUEST, "Missing PKCE verifier".into())),
    };
    let nonce: Nonce = match session.get::<Nonce>("nonce").await {
        Ok(Some(v)) => v,
        _ => return Err((axum::http::StatusCode::BAD_REQUEST, "Missing nonce".into())),
    };

    let token_response = match app_state
        .oidc_client
        .exchange_code(AuthorizationCode::new(params.code))
        .map_err(|e| {eprintln!("Token exchange network error: {:?}", e);
        (axum::http::StatusCode::BAD_REQUEST, "HTTP error on code exchange".into())})?
        .set_pkce_verifier(pkce_verifier)
        .request_async(&app_state.http_client)
        .await
    {
        Ok(v) => v,
        _ => return Err((axum::http::StatusCode::BAD_REQUEST, "Failed to exchange code for tokens".into())),
    };
    let id_token =
        token_response
            .id_token()
            .ok_or((axum::http::StatusCode::BAD_REQUEST, "Server did not return an ID token".into()))?;

    let id_token_verifier = app_state.oidc_client.id_token_verifier();

    let claims = id_token.claims(&id_token_verifier, &nonce).map_err(|_| {
        (axum::http::StatusCode::BAD_REQUEST, "Failed to verify ID token claims".into())
    })?;
    session.remove::<String>("csrf_token").await.ok();
    session.remove::<String>("nonce").await.ok();
    session.remove::<String>("pkce_verifier").await.ok();
    session.insert("user_sub", claims.subject().as_str()).await.unwrap();

    Ok(Redirect::to("/user"))



}

