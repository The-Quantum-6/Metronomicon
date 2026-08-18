use crate::error::{AppError, RequestError};
use crate::middleware::jwt::AuthUser;
use crate::models::user::UserRole;
use crate::state::AppState;
use crate::storage::Storage;
use axum::extract::{DefaultBodyLimit, Multipart, Path, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::{Json, Router, routing::get, routing::post};
use serde::Serialize;
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
    /// The storage key the file was saved under. Persist this (e.g. in an event)
    /// to refer to the file later.
    key: String,
}

/// Public file routes. Uploading is open because a freshly minted key is an
/// unguessable UUID that nothing links to until a contribution is approved,
/// and downloads need to work for anyone who can already see a resource.
pub fn router() -> Router<AppState> {
    let upload_max_bytes = upload_max_bytes();
    Router::new()
        .route(
            "/files",
            post(upload_file).layer(DefaultBodyLimit::max(upload_max_bytes)),
        )
        .route("/files/{key}", get(download_file))
}

/// Listing every key is admin-only: it would otherwise expose the keys of
/// files still awaiting moderation, or already denied.
pub fn protected_router() -> Router<AppState> {
    Router::new().route("/files", get(list_files))
}

/// Uploads a file sent as `multipart/form-data` under the field name `file`,
/// returning the generated storage key.
async fn upload_file(
    State(storage): State<Storage>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
    while let Some(field) = multipart.next_field().await? {
        if field.name() != Some("file") {
            continue;
        }

        let content_type = field.content_type().map(|s| s.to_string());
        let filename = field.file_name().unwrap_or("").to_string();
        // Generate our own key so we never trust the client's filename, but keep
        // the extension (if any) so the object stays recognisable.
        let key = match filename.rsplit_once('.') {
            Some((_, ext)) if !ext.is_empty() => format!("{}.{}", Uuid::new_v4(), ext),
            _ => Uuid::new_v4().to_string(),
        };

        let bytes = field.bytes().await?.to_vec();
        storage.upload(&key, bytes, content_type.as_deref()).await?;
        return Ok(Json(UploadResponse { key }));
    }

    // The form had no field named "file".
    Err(AppError::BadRequest(RequestError::NonExsistant("file")))
}

/// Downloads the object stored under `key`.
async fn download_file(
    State(storage): State<Storage>,
    Path(key): Path<String>,
) -> Result<Response, AppError> {
    if !storage.exists(&key).await? {
        return Err(AppError::BadRequest(RequestError::NonExsistant("file")));
    }
    let bytes = storage.download(&key).await?;
    // Content types are not tracked yet, so serve as a generic download.
    Ok(([(header::CONTENT_TYPE, "application/octet-stream")], bytes).into_response())
}

/// Lists the keys of all stored objects. Admins only — see `protected_router`.
async fn list_files(
    AuthUser(claims): AuthUser,
    State(storage): State<Storage>,
) -> Result<Json<Vec<String>>, Response> {
    if !matches!(claims.role, UserRole::Admin | UserRole::Root) {
        return Err(StatusCode::FORBIDDEN.into_response());
    }
    let keys = storage
        .list(None)
        .await
        .map_err(|e| AppError::from(e).into_response())?;
    Ok(Json(keys))
}
