use axum::{Router, routing::get};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/testuser", get(user_test))
}

async fn user_test() -> String {
    "User test endpoint".to_string()
}
