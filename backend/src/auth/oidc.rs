use crate::{
    repositories::user::{create_user, get_user_by_sub},
    state::AppState,
};
use axum::http::{HeaderValue, header};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use openidconnect::core::CoreAuthenticationFlow;
use openidconnect::{
    AccessTokenHash, AuthorizationCode, CsrfToken, Nonce, OAuth2TokenResponse, PkceCodeChallenge,
    Scope, TokenResponse,
};
use serde::Deserialize;
use tower_sessions::Session;
use uuid::Uuid;

use crate::auth::jwt::generate_access;

#[derive(Deserialize)]
pub struct CallbackParams {
    code: String,
    state: String,
}

pub async fn login_send(
    session: Session,
    State(client): State<AppState>,
) -> Result<Redirect, (axum::http::StatusCode, String)> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token, nonce) = client
        .oidc_client
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

    session
        .insert("csrf_token", csrf_token.secret())
        .await
        .map_err(|e| {
            eprintln!("Session error: {:?}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to save CSRF token".into(),
            )
        })?;
    session.insert("nonce", nonce.secret()).await.map_err(|e| {
        eprintln!("Session error: {:?}", e);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to save CSRF token".into(),
        )
    })?;
    session
        .insert("pkce_verifier", pkce_verifier.secret())
        .await
        .map_err(|e| {
            eprintln!("Session error: {:?}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to save CSRF token".into(),
            )
        })?;

    Ok(Redirect::to(auth_url.as_str()))
}

pub async fn login_callback(
    Query(params): Query<CallbackParams>,
    State(app_state): State<AppState>,
    session: Session,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    let stored_csrf: String = match session.get("csrf_token").await {
        Ok(Some(v)) => v,
        _ => {
            return Err((
                axum::http::StatusCode::BAD_REQUEST,
                "Missing CSRF token".into(),
            ));
        }
    };
    if stored_csrf != params.state {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            "CSRF token mismatch".into(),
        ));
    }
    let pkce_verifier = match session.get("pkce_verifier").await {
        Ok(Some(v)) => v,
        _ => {
            return Err((
                axum::http::StatusCode::BAD_REQUEST,
                "Missing PKCE verifier".into(),
            ));
        }
    };
    let nonce: Nonce = match session.get::<Nonce>("nonce").await {
        Ok(Some(v)) => v,
        _ => return Err((axum::http::StatusCode::BAD_REQUEST, "Missing nonce".into())),
    };

    let token_response = match app_state
        .oidc_client
        .exchange_code(AuthorizationCode::new(params.code))
        .map_err(|e| {
            eprintln!("Token exchange network error: {:?}", e);
            (
                axum::http::StatusCode::BAD_REQUEST,
                "HTTP error on code exchange".into(),
            )
        })?
        .set_pkce_verifier(pkce_verifier)
        .request_async(&app_state.http_client)
        .await
    {
        Ok(v) => v,
        _ => {
            return Err((
                axum::http::StatusCode::BAD_REQUEST,
                "Failed to exchange code for tokens".into(),
            ));
        }
    };
    let id_token = token_response.id_token().ok_or((
        axum::http::StatusCode::BAD_REQUEST,
        "Server did not return an ID token".into(),
    ))?;

    let id_token_verifier = app_state.oidc_client.id_token_verifier();

    let claims = id_token.claims(&id_token_verifier, &nonce).map_err(|_| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            "Failed to verify ID token claims".into(),
        )
    })?;

    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let actual_access_token_hash = AccessTokenHash::from_token(
            token_response.access_token(),
            id_token.signing_alg().map_err(|_| {
                (
                    axum::http::StatusCode::BAD_REQUEST,
                    "Failed to get signing algorithm".into(),
                )
            })?,
            id_token.signing_key(&id_token_verifier).map_err(|_| {
                (
                    axum::http::StatusCode::BAD_REQUEST,
                    "Failed to get signing key".into(),
                )
            })?,
        )
        .map_err(|_| {
            (
                axum::http::StatusCode::BAD_REQUEST,
                "Failed to create access token hash".into(),
            )
        })?;
        if actual_access_token_hash != *expected_access_token_hash {
            return Err((
                axum::http::StatusCode::BAD_REQUEST,
                "Access token hash mismatch".into(),
            ));
        }
    }
    let user_sub: String = claims.subject().as_str().to_string();
    let user_name: String = claims
        .name()
        .and_then(|localized| localized.get(None))
        .map(|name_type| name_type.to_string())
        .unwrap_or_else(|| "user".to_string());

    let user_email: Option<String> = claims.email().map(|email| email.as_str().to_string());

    session.remove::<String>("csrf_token").await.ok();
    session.remove::<String>("nonce").await.ok();
    session.remove::<String>("pkce_verifier").await.ok();

    let user_record = get_user_by_sub(&app_state.pool, user_sub.clone())
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database lookup error: {}", e),
            )
        })?;

    if user_record.is_none() {
        create_user(&app_state.pool, user_sub.clone(), user_name, user_email)
            .await
            .map_err(|e| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to create user profile: {}", e),
                )
            })?;
    }

    let user = get_user_by_sub(&app_state.pool, user_sub.clone())
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database lookup error: {}", e),
            )
        })?
        .ok_or((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "User not found after creation".into(),
        ))?;

    let token =
        generate_access(&app_state.jwt_encode, user.role, user.id.to_string()).map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to generate token: {}", e),
            )
        })?;

    let cookie = format!("access_token={}; HttpOnly; SameSite=Lax; Path=/", token);

    let new_refresh = Uuid::new_v4().to_string();
    session.insert("refresh_token", new_refresh).await.ok();
    session.insert("user_id", user.id).await.ok();

    let mut response = Redirect::to("/me").into_response();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());

    Ok(response)
}
