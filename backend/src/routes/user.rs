use crate::state::AppState;
use axum::{Router, routing::get};

pub fn router() -> Router<AppState> {
    Router::new().route("/testuser", get(user_test))
}

async fn user_test() -> String {
    "User test endpoint".to_string()
}
