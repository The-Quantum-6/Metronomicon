use crate::extractors::resource::ResourceCommandExtractor;
use crate::state::AppState;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::post;

pub fn router() -> Router<AppState> {
    Router::new().route("/resources", post(handle_command))
}

pub async fn handle_command(
    State(state): State<AppState>,
    ResourceCommandExtractor(metadata, command): ResourceCommandExtractor,
) -> Response {
    let resource_id = command.id();

    match state
        .cqrs
        .resource
        .execute_with_metadata(&resource_id.to_string(), command, metadata)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            println!("Error: {e:#?}\n");
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
    }
}
