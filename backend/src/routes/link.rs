use crate::extractors::link::LinkCommandExtractor;
use crate::state::AppState;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::post;

pub fn router() -> Router<AppState> {
    Router::new().route("/links", post(handle_command))
}

pub async fn handle_command(
    State(state): State<AppState>,
    LinkCommandExtractor(metadata, command): LinkCommandExtractor,
) -> Response {
    let link_id = command.id();

    match state
        .cqrs
        .link
        .execute_with_metadata(&link_id.to_string(), command, metadata)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            println!("Error: {e:#?}\n");
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
    }
}
