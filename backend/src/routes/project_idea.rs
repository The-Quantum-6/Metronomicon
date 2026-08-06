use crate::extractors::project_idea::ProjectIdeaCommandExtractor;
use crate::middleware::perms::perm_middleware;
use crate::state::AppState;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::post;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new().route("/project_idea", post(handle_command))
}

pub async fn handle_command(
    State(state): State<AppState>,
    ProjectIdeaCommandExtractor(metadata, command): ProjectIdeaCommandExtractor,
) -> Response {
    let idea_id = command.id();

    match state
        .cqrs
        .project_idea
        .execute_with_metadata(&idea_id.to_string(), command, metadata)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            println!("Error: {e:#?}\n");
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
    }
}
