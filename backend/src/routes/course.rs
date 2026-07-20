use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;
use serde::Serialize;

use crate::{extractors::course::CourseCommandExtractor, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/courses", get(list_active_courses).post(handle_command))
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
    match state.course_view_repo.load_active(&course_id).await {
        Ok(Some(course_view)) => (StatusCode::OK, Json(course_view)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => {
            println!("Error: {err:#?}\n");
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn list_active_courses(
    State(state): State<AppState>,
) -> Result<Json<Vec<CourseDTO>>, StatusCode> {
    Ok::<axum::Json<Vec<CourseDTO>>, StatusCode>(Json(
        sqlx::query_as!(
            CourseDTO,
            r#"
        SELECT aggregate_id, name, code, field
        FROM course_list_view
        WHERE status = 'Active'
        ORDER BY name
        "#
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}

#[derive(Serialize, Deserialize)]
pub struct CourseDTO {
    aggregate_id: String,
    name: String,
    code: String,
    field: String,
}
