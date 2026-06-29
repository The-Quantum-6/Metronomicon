use crate::error::{AppError, RequestError};
use crate::state::AppState;
use crate::storage::Storage;
use axum::extract::{Multipart, Path, State};
use axum::http::header;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router, routing::get};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct UploadResponse {
    /// The storage key the file was saved under. Persist this (e.g. in an event)
    /// to refer to the file later.
    key: String,
}

/// File upload/download routes, mounted under `/files`.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/files", get(list_files).post(upload_file))
        .route("/files/{key}", get(download_file))
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
    let bytes = storage.download(&key).await?;
    // Content types are not tracked yet, so serve as a generic download.
    Ok(([(header::CONTENT_TYPE, "application/octet-stream")], bytes).into_response())
}

/// Lists the keys of all stored objects.
async fn list_files(State(storage): State<Storage>) -> Result<Json<Vec<String>>, AppError> {
    Ok(Json(storage.list(None).await?))
}
