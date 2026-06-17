use axum::{Router, extract::Query, response::Redirect, routing::get};
use base64::{Engine, engine::general_purpose::STANDARD};
use reqwest::Client;
use serde::{Deserialize};
use sqlx::PgPool;
use rand::distr::Alphanumeric;
use rand::RngExt;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/login/feide", get(login_send))
        .route("/login/callback", get(login_callback))
}

#[derive(Deserialize)]
struct CallbackCode {
    code: String,
    state: String,
}

#[derive(Deserialize)]
struct AccessToken {
    access_token: String,
}

async fn login_send() -> Redirect {
    let client_id: String = std::env::var("FEIDE_CLIENT_ID").expect("FEIDE_CLIENT_ID must be set");
    let redirect_uri: &str = "http://localhost:3000/login/callback";
    let response_type: &str = "code";
    let scope: &str = "openid";

    let rand: String = rand::rng()
        .sample_iter(Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();
    let state: &str = &rand;
    let auth_url: String = format!(
        "https://auth.dataporten.no/oauth/authorization?response_type={}&client_id={}&redirect_uri={}&scope={}&state={}",
        response_type, client_id, redirect_uri, scope, state
    );

    Redirect::temporary(&auth_url)
}

async fn login_callback(Query(params): Query<CallbackCode>) -> Redirect {
    let code: String = params.code;
    if !code.is_empty() {
        let access_token: AccessToken = get_token(code).await;
        // redirect to only /user, then get accesstoken from cookies in future
        return Redirect::temporary(&format!("/user?access_token={}", access_token.access_token));
    }
    Redirect::temporary("/login")
}

async fn get_token(code: String) -> AccessToken {
    let client_id: String = std::env::var("FEIDE_CLIENT_ID").expect("FEIDE_CLIENT_ID must be set");
    let client_secret: String = std::env::var("FEIDE_SECRET").expect("FEIDE_SECRET must be set");
    let redirect_uri: &str = "http://localhost:3000/login/callback";
    let token_url: &str = "https://auth.dataporten.no/oauth/token";

    let credentials = STANDARD.encode(format!("{}:{}", client_id, client_secret));

    let client: Client = Client::new();
    let response: reqwest::Response = client
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

    response
        .json()
        .await
        .expect("Token response should be valid JSON")
}
