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

async fn login_send(session: Session, State(client): State<AppState>) -> Redirect {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token, nonce) = client
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

    session.insert("pkce_verifier", pkce_verifier).await.expect("Should store PKCE verifier in session");
    session.insert("csrf_token", csrf_token.clone()).await.expect("Should store CSRF token in session");
    session.insert("nonce", nonce.clone()).await.expect("Should store nonce in session");


    Redirect::to(auth_url.as_str())
}

async fn login_callback(Query(params): Query<CallbackParams>, State(client): State<AppState>, session: Session) -> Result<Redirect, (axum::http::StatusCode, String)> {
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
    let nonce = match session.get::<Nonce>("nonce").await {
        Ok(Some(v)) => v,
        _ => return Err((axum::http::StatusCode::BAD_REQUEST, "Missing nonce".into())),
    };

    let token_response = match state.oidc_client
        .exchange_code(AuthorizationCode::new(params.code))
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Token exchange failed".into()))?
        .set_pkce_verifier(pkce_verifier)
        .request_async(&state.http_client)
        .await
    {
        Ok(t) => t,
        Err(_) => {
            return Err((axum::http::StatusCode::BAD_REQUEST, "HTTP error on code exchange".into()));
        }
    };
    let id_token =
        token_response
            .id_token()
            .ok_or((axum::http::StatusCode::BAD_REQUEST, "Server did not return an ID token".into()))?;

    let id_token_verifier = state.oauth_client.id_token_verifier();

    let claims = id_token.claims(&id_token_verifier, &nonce).map_err(|_| {
        (axum::http::StatusCode::BAD_REQUEST, "Failed to verify ID token claims".into())
    })?;
    session.remove::<String>("csrf_token").await.ok();
    session.remove::<String>("nonce").await.ok();
    session.remove::<String>("pkce_verifier").await.ok();
    session.insert("user_sub", claims.subject().as_str()).await.unwrap();

    Ok(Redirect::to("/user"))



}

