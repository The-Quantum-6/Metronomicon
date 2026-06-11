use axum::{Router, routing::get, extract::{Query}, response::Redirect};
use base64::{Engine, engine::general_purpose::STANDARD};
use reqwest::Client;
use serde::Deserialize;
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/login", get(login_send))
        .route("/login/callback", get(login_callback))
        .route("/user", get(user_info))
}

#[derive(Deserialize)]
struct CallbackCode {
    code: String,
}

#[derive(Deserialize)]
struct AccessToken {
    access_token: String,
}


async fn login_send() -> Redirect {
    let client_id = std::env::var("FEIDE_CLIENT_ID").expect("FEIDE_CLIENT_ID must be set");
    let redirect_uri = "http://localhost:3000/login/callback";
    let response_type="code";
    let scope="openid";
    let state="whatever";
    let auth_url = format!(
        "https://auth.dataporten.no/oauth/authorization?response_type={}&client_id={}&redirect_uri={}&scope={}&state={}",
        response_type, client_id, redirect_uri, scope, state
    );

    Redirect::temporary(&auth_url)
}

async fn login_callback(Query(params): Query<CallbackCode>) -> Redirect { 
    let code = params.code;
    if !code.is_empty() {
        let access_token = get_token(code).await;
        // redirect to only /user, then get accesstoken from cookies in future
        return Redirect::temporary(&format!("/user?access_token={}", access_token.access_token));
    }
    Redirect::temporary("/login")
}

async fn user_info(Query(params): Query<AccessToken>) -> String {
    let token = params.access_token;
    if !token.is_empty() {
        fetch_user_info(token).await;
    }
    "Invalid token".into()
}

async fn get_token(code: String) -> AccessToken {
    let client_id = std::env::var("FEIDE_CLIENT_ID").expect("FEIDE_CLIENT_ID must be set");
    let client_secret = std::env::var("FEIDE_SECRET").expect("FEIDE_SECRET must be set");
    let redirect_uri = "http://localhost:3000/login/callback";
    let token_url = "https://auth.dataporten.no/oauth/token";

    let credentials = STANDARD.encode(format!("{}:{}", client_id, client_secret));

    let client = Client::new();
    let response = client
        .post(token_url)
        .header("Authorization", format!("Basic {}", credentials))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[
            ("client_id", client_id.as_str()),
            ("grant_type", "authorization_code"),
            ("code", code.as_str()),
            ("redirect_uri", redirect_uri),
        ])
        .send()
        .await
        .expect("Token request should succeed");

    response.json().await.expect("Token response should be valid JSON");
}

async fn fetch_user_info(token: String) -> String { 
    let endpoint = "https://auth.dataporten.no/openid/userinfo";
    let client = Client::new();
    let response = client
        .get(endpoint)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("User info request should succeed");

    response.text().await.unwrap_or_default();
}