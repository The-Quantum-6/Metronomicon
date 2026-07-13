use crate::extractors::faq::FaqCommandExtractor;
use crate::state::AppState;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::post;

pub fn router() -> Router<AppState> {
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
