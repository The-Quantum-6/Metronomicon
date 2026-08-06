use crate::error::{AppError, RequestError};
use crate::handlers::perms::is_super_user;
use crate::middleware::jwt::{AuthUser, jwt_middleware};
use crate::models::permissions::Permissions;
use crate::queries::resource::resource_key_is_active;
use crate::repositories::permissions::{default_permissions, get_user_permissions};
use crate::state::AppState;
use axum::extract::{DefaultBodyLimit, Multipart, Path, Query, State};
use axum::http::header;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router, middleware};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const DEFAULT_UPLOAD_MAX_BYTES: usize = 10 * 1024 * 1024; // 10 MiB

fn upload_max_bytes() -> usize {
    std::env::var("UPLOAD_MAX_BYTES")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_UPLOAD_MAX_BYTES)
}

#[derive(Serialize)]
struct UploadResponse {
    /// The storage key the file was saved under. Persist this (e.g. in a
    /// contribution) to refer to the file later.
    key: String,
}

#[derive(Deserialize)]
struct CourseIdQuery {
    course_id: String,
}

/// Public routes: gated file download only. Anyone can hit this — the gate
/// (`resource_key_is_active`) is what keeps pending uploads private, not auth.
pub fn router() -> Router<AppState> {
    Router::new().route("/files/{key}", get(download_file))
}

/// Upload, bucket listing, and moderator preview all need to know who's
/// asking, so they sit behind `jwt_middleware`. They deliberately do NOT go
/// through `perm_middleware`: that middleware parses the body as JSON to
/// look up the command name, which breaks on multipart uploads (every
/// request would be rejected with 403). Permissions are checked by hand
/// below instead.
pub fn protected_router(state: AppState) -> Router<AppState> {
    let upload_max_bytes = upload_max_bytes();
    Router::new()
        .route(
            "/files",
            get(list_files)
                .post(upload_file)
                .layer(DefaultBodyLimit::max(upload_max_bytes)),
        )
        .route("/files/{key}/preview", get(preview_file))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            jwt_middleware,
        ))
}

/// Uploads a file sent as `multipart/form-data` with fields `file` and
/// `course_id`, returning the generated storage key. Requires `SUGGEST_FILE`
/// on the given course — the same permission `Contribution::Propose` checks
/// for file contributions, since an upload with no matching contribution is
/// just an orphaned object in Garage.
async fn upload_file(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
    let mut course_id: Option<String> = None;
    let mut file_field: Option<(String, Option<String>, Vec<u8>)> = None;

    while let Some(field) = multipart.next_field().await? {
        match field.name() {
            Some("course_id") => {
                course_id = Some(field.text().await?);
            }
            Some("file") => {
                let filename = field.file_name().unwrap_or("").to_string();
                let content_type = field.content_type().map(|s| s.to_string());
                let bytes = field.bytes().await?.to_vec();
                file_field = Some((filename, content_type, bytes));
            }
            _ => {}
        }
    }

    let course_id =
        course_id.ok_or(AppError::BadRequest(RequestError::NonExsistant("course_id")))?;
    let (filename, content_type, bytes) =
        file_field.ok_or(AppError::BadRequest(RequestError::NonExsistant("file")))?;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::BadRequest(RequestError::NonExsistant("user")))?;

    default_permissions(&state.pool, user_id, &course_id).await?;
    let perms = get_user_permissions(&state.pool, user_id, &course_id).await?;
    if !perms.contains(Permissions::SUGGEST_FILE) {
        return Err(AppError::Forbidden);
    }

    // Generate our own key so we never trust the client's filename, but keep
    // the extension (if any) so the object stays recognisable.
    let key = match filename.rsplit_once('.') {
        Some((_, ext)) if !ext.is_empty() => format!("{}.{}", Uuid::new_v4(), ext),
        _ => Uuid::new_v4().to_string(),
    };

    state
        .storage
        .upload(&key, bytes, content_type.as_deref())
        .await?;
    Ok(Json(UploadResponse { key }))
}

/// Downloads the object stored under `key` — but only if it belongs to an
/// approved, active resource. Files sitting in Garage from a not-yet-approved
/// (or denied/never-proposed) upload have no matching resource, so they 404
/// here even though the bytes exist in storage.
async fn download_file(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Response, AppError> {
    if !resource_key_is_active(&state.pool, &key).await? {
        return Err(AppError::BadRequest(RequestError::NonExsistant("file")));
    }
    let bytes = state.storage.download(&key).await?;
    // Content types are not tracked yet, so serve as a generic download.
    Ok(([(header::CONTENT_TYPE, "application/octet-stream")], bytes).into_response())
}

/// Lets a moderator preview a file that's still pending approval (and so
/// isn't reachable via `download_file` yet). Requires `MODERATE_FILE` on the
/// contribution's course — passed as a query param since a pending upload
/// has no resource record to look the course up from.
async fn preview_file(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(key): Path<String>,
    Query(q): Query<CourseIdQuery>,
) -> Result<Response, AppError> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::BadRequest(RequestError::NonExsistant("user")))?;

    default_permissions(&state.pool, user_id, &q.course_id).await?;
    let perms = get_user_permissions(&state.pool, user_id, &q.course_id).await?;
    if !perms.contains(Permissions::MODERATE_FILE) {
        return Err(AppError::Forbidden);
    }

    let bytes = state.storage.download(&key).await?;
    Ok(([(header::CONTENT_TYPE, "application/octet-stream")], bytes).into_response())
}

/// Lists every key in the bucket, pending or approved. Not course-scoped
/// (it's a raw bucket dump), so it's restricted to staff rather than gated
/// per-course like the other file routes.
async fn list_files(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
) -> Result<Json<Vec<String>>, AppError> {
    if !is_super_user(&claims.role) {
        return Err(AppError::Forbidden);
    }
    Ok(Json(state.storage.list(None).await?))
}
