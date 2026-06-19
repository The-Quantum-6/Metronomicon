use crate::models::course::Course;
use crate::repositories::course as course_repo;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::{Router, routing::delete, routing::get, routing::post};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/courses", get(get_courses))
        .route("/courses/create", post(create_course))
        .route("/courses/{id}", get(get_course_by_id))
        .route("/courses/code/{code}", get(get_course_by_code))
        .route("/courses/{id}", delete(delete_course))
}

#[derive(Deserialize)]
struct CourseCreateRequest {
    name: String,
    content: Option<String>,
    code: String,
}

async fn get_courses(State(pool): State<PgPool>) -> Result<Json<Vec<Course>>, StatusCode> {
    course_repo::get_courses(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(|v| Json(v))
}

async fn create_course(
    State(pool): State<PgPool>,
    Json(course): Json<CourseCreateRequest>,
) -> Result<(), StatusCode> {
    course_repo::create_course(&pool, course.name, course.content, course.code)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_course_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Course>, StatusCode> {
    let course = course_repo::get_course_by_id(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(course))
}

async fn get_course_by_code(
    State(pool): State<PgPool>,
    Path(code): Path<String>,
) -> Result<Json<Course>, StatusCode> {
    let course = course_repo::get_course_by_code(&pool, &code)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(course))
}

async fn delete_course(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Result<(), StatusCode> {
    course_repo::delete_course(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
