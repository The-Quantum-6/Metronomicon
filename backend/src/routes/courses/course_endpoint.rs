use crate::error::{AppError, RequestError};
use crate::models::course::Course;
use crate::repositories::course as course_repo;
use axum::extract::{Json, Path, State};
use axum::{Router, routing::delete, routing::get, routing::post, routing::put};
use serde::Deserialize;
use sqlx::PgPool;
use crate::state::AppState;
use uuid::Uuid;

/// Routes for course CRUD operations, mounted under `/courses`.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/courses", get(get_courses))
        .route("/courses/create", post(create_course))
        .route("/courses/{id}", get(get_course_by_id))
        .route("/courses/code/{code}", get(get_course_by_code))
        .route("/courses/{id}", delete(delete_course))
        .route("/courses/{id}", put(update_course))
}

#[derive(Deserialize)]
struct CourseCreateRequest {
    name: String,
    content: Option<String>,
    code: String,
}

#[derive(Deserialize)]
struct CourseUpdateRequest {
    name: String,
    content: Option<String>,
    code: String,
}

/// Fetches all courses.
async fn get_courses(State(pool): State<PgPool>) -> Result<Json<Vec<Course>>, AppError> {
    Ok(Json(course_repo::get_courses(&pool).await?))
}

/// Creates a new course.
async fn create_course(
    State(pool): State<PgPool>,
    Json(course): Json<CourseCreateRequest>,
) -> Result<(), AppError> {
    Ok(course_repo::create_course(&pool, course.name, course.content, course.code).await?)
}

/// Fetches a course by its id, returning a `BadRequest` if none exists.
async fn get_course_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Course>, AppError> {
    let course = course_repo::get_course_by_id(&pool, id)
        .await?
        .ok_or(AppError::BadRequest(RequestError::NonExsistant("Course")))?;

    Ok(Json(course))
}

/// Fetches a course by course code. There is no guarantee that a specific instance of course is fetched if several match the code. This is not enforced unique in the db.
async fn get_course_by_code(
    State(pool): State<PgPool>,
    Path(code): Path<String>,
) -> Result<Json<Course>, AppError> {
    let course = course_repo::get_course_by_code(&pool, &code)
        .await?
        .ok_or(AppError::BadRequest(RequestError::NonExsistant("Course")))?;

    Ok(Json(course))
}

/// Deletes a course by its id.
async fn delete_course(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Result<(), AppError> {
    Ok(course_repo::delete_course(&pool, id).await?)
}

/// Updates an existing course by its id.
async fn update_course(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(course): Json<CourseUpdateRequest>,
) -> Result<(), AppError> {
    Ok(course_repo::update_course(&pool, id, course.name, course.content, course.code).await?)
}
