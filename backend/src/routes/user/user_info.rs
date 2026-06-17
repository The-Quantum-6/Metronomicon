use axum::{Json, Router, extract::Query, routing::get};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::state::AppState;
pub fn router() -> Router<AppState> {
    Router::new().route("/user", get(user_info))
}

#[derive(Deserialize, Serialize)]
struct User {
    sub: String,
    name: String,
}

#[derive(Deserialize)]
struct AccessToken {
    access_token: String,
}

async fn user_info(session: Session) -> Json<User> {
    let userinfo = session.get::<String>("user_sub").await.unwrap_or(None);
    if let Some(sub) = userinfo {
        return Json(User {
            sub: sub.clone(),
            name: format!("User {}", sub),
        });
    }
    Json(User {
        sub: "unknown".to_string(),
        name: "Unknown User".to_string(),
    })
}

async fn fetch_user_info(token: String) -> User {
    let endpoint: &str = "https://auth.dataporten.no/openid/userinfo";
    let client: Client = Client::new();
    let response: reqwest::Response = client
        .get(endpoint)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("User info request should succeed");

    response
        .json()
        .await
        .expect("User info response should be valid JSON")
}
