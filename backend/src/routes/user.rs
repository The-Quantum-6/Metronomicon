use axum::{Router, routing::get, extract::{Query}, Json};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use reqwest::Client;

pub fn router() -> Router<PgPool> {
    Router::new()
    .route("/user", get(user_info))

}

#[derive(Deserialize, Serialize)]
struct User{
    sub: String,
    name: String,
}

#[derive(Deserialize)]
struct AccessToken {
    access_token: String,
}

async fn user_info(Query(params): Query<AccessToken>) -> Json<User> {
    let token: String = params.access_token;
    if !token.is_empty() {
        let user: User = fetch_user_info(token).await;
        return Json(user);
    }
    Json(User { sub: "unknown".to_string(), name: "Unknown User".to_string() })
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

    response.json().await.expect("User info response should be valid JSON")
}