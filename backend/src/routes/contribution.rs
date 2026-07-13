use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::post;
use axum::extract::State;

use crate::extractors::contribution::ContributionCommandExtractor;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/contributions", post(handle_command))
}

pub async fn handle_command(
    State(state): State<AppState>,
    ContributionCommandExtractor(metadata, command): ContributionCommandExtractor,
) -> Response {
    let contribution_id = command.id();

    match state
        .cqrs
        .contribution
        .execute_with_metadata(&contribution_id.to_string(), command, metadata)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            println!("Error: {e:#?}\n");
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
    }
}
