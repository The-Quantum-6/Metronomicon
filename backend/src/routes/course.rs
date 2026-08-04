use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::{get, post};
use axum::{
    Json,
    extract::{Path, State, Query},
};
use serde::Deserialize;
use serde::Serialize;

use crate::{
    extractors::course::CourseCommandExtractor,
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/courses", get(list_courses))
        .route("/courses/{id}", get(query_handler))
}

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .route("/courses", post(handle_command))
        .route("/courses/{id}/activate", post(activate_course))
        .route("/courses/{id}/unactivate", post(unactivate_course))
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

pub async fn activate_course(
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

pub async fn unactivate_course(
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

#[derive(Deserialize)]
pub struct CourseListQuery {
    status: Option<String>,
}

pub async fn list_courses(
    Query(query): Query<CourseListQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<CourseDTO>>, StatusCode> {
    let status = query.status.as_deref().unwrap_or("all");

    let query_str = match status {
        "active" => r#"
            SELECT aggregate_id, name, code, field
            FROM course_list_view
            WHERE status = 'Active'
            ORDER BY name
        "#,
        "unactive" => r#"
            SELECT aggregate_id, name, code, field
            FROM course_list_view
            WHERE status = 'Unactive'
            ORDER BY name
        "#,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let courses = sqlx::query_as::<_, CourseDTO>(query_str)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(courses))
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct CourseDTO {
    aggregate_id: String,
    name: String,
    code: String,
    field: String,
}
