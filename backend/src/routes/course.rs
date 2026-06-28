use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::routing::post;
use axum::{
    Json,
    extract::{Path, State},
};
use cqrs_es::persist::ViewRepository;

use crate::{extractors::course::CourseCommandExtractor, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/courses", post(handle_command))
        .route("/courses/{id}", get(query_handler))
}

pub async fn handle_command(
    State(state): State<AppState>,
    CourseCommandExtractor(metadata, command): CourseCommandExtractor,
) -> Response {
    let course_id = command.id();

    match state
        .cqrs
        .course
        .execute_with_metadata(&course_id.to_string(), command, metadata)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            println!("Error: {e:#?}\n");
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
    }
}

pub async fn query_handler(
    Path(course_id): Path<String>,
    State(state): State<AppState>,
) -> Response {
    match state.course_view_repo.load(&course_id).await {
        Ok(Some(course_view)) => (StatusCode::OK, Json(course_view)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => {
            println!("Error: {err:#?}\n");
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}
