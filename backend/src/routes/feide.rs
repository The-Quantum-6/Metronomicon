use axum::{Router, routing::get, routing::post, extract::{Query, Json}, response::Redirect};
use base64::{Engine, engine::general_purpose::STANDARD};
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

async fn login_callback(Query(params): Query<CallbackCode>) -> String { 
    let code = params.code;
    if !code.is_empty() {
        return get_token(code).await;
    }
    "Invalid code".into()
}

async fn get_token(code: String) -> String {
    let client_id = std::env::var("FEIDE_CLIENT_ID").expect("FEIDE_CLIENT_ID must be set");
    let client_secret = std::env::var("FEIDE_SECRET").expect("FEIDE_SECRET must be set");
    let redirect_uri = "http://localhost:3000/login/callback";
    let token_url = "https://auth.dataporten.no/oauth/token";

    let credentials = STANDARD.encode(format!("{}:{}", client_id, client_secret));

    let client = reqwest::Client::new();
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

    return user_info(response.json::<serde_json::Value>().await.expect("Token response should be JSON")["access_token"].as_str().unwrap().to_string()).await;
}
async fn user_info(token: String) -> serde_json::Value { 
    let endpoint = "https://auth.dataporten.no/openid/userinfo";
    let client = reqwest::Client::new();
    let response = client
        .get(endpoint)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("User info request should succeed");

    return response.json().await.expect("User info");
}