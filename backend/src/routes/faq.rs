use crate::extractors::faq::FaqCommandExtractor;
use crate::middleware::jwt::jwt_middleware;
use crate::middleware::perms::perm_middleware;
use crate::models::permissions::Permissions;
use crate::state::AppState;
use axum::Extension;
use axum::Router;
use axum::extract::Request;
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::post;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new().route("/faqs", post(handle_command))
}

pub async fn handle_command(
    State(state): State<AppState>,
    FaqCommandExtractor(metadata, command): FaqCommandExtractor,
) -> Response {
    let faq_id = command.id();

    match state
        .cqrs
        .faq
        .execute_with_metadata(&faq_id.to_string(), command, metadata)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            println!("Error: {e:#?}\n");
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
    }
}
