use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

/// The single, top-level error type for the application.
///
/// `AppError` implements [`IntoResponse`], so any handler can simply return
/// `Result<T, AppError>` and axum will turn an `Err` into a proper HTTP
/// response automatically.
///
/// # Example
/// ```
/// async fn some_handler() -> Result<(), AppError> {
///     // The `?` operator works here because of `#[from] sqlx::Error`
///     // on the `Database` variant below — it converts a `sqlx::Error`
///     // into an `AppError` automatically.
///     sqlx::query("SELECT 1").execute(&pool).await?;
///     Ok(())
/// }
/// ```
///
/// # Adding a new error variant
/// 1. Add a variant below with an `#[error("...")]` message (this is the
///    *internal* / log-facing message, not what the client sees).
/// 2. If it wraps another error type, add `#[from]` so `?` works for it.
/// 3. Add a matching arm in `IntoResponse for AppError` below, mapping it
///    to a `StatusCode` and a *client-facing* message.
#[derive(Error, Debug)]
pub enum AppError {
    /// Any failure coming from the database layer (query errors, pool
    /// errors, connection failures, etc). `#[from]` lets you use `?`
    /// directly on `sqlx::Result` values inside handlers.
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("bad request: {0}")]
    BadRequest(#[from] RequestError),
    // --- Template for future variants — copy one of these when you need it ---
    //
    // /// Session store (e.g. Tower Sessions) failed to read/write a session.
    // #[error("session storage failed: {0}")]
    // Session(#[from] tower_sessions::session::Error),
    //
    // /// Authorization/permission failure. Wraps a more specific error type
    // /// defined elsewhere in the project (e.g. `auth::AuthError`), which
    // /// keeps that module's error details out of this top-level enum.
    // #[error("authorization error: {0}")]
    // Auth(#[from] AuthError),
}

/// Error related to malformed request.
#[derive(Error, Debug)]
pub enum RequestError {
    /// Request was for a non existant resource
    #[error("Non exsistant resource: {0}")]
    NonExsistant(&'static str),
}

impl IntoResponse for AppError {
    /// Converts an `AppError` into an HTTP response.
    ///
    /// Important: the message shown here is what the *client* sees, so it
    /// should stay generic and never leak internals (SQL text, file paths,
    /// stack traces, etc). The detailed `#[error(...)]` message from above
    /// is for logs only — we can log it here with `tracing` before discarding it.
    fn into_response(self) -> axum::response::Response {
        // Here, we can log the real error for debugging/observability. `self` still has
        // the full error chain at this point (via `{:?}` / `{}`).
        // tracing::error!(error = ?self, "request failed with AppError");

        let (status, client_message) = match self {
            Self::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong on our end. Please try again later.",
            ),
            Self::BadRequest(_) => (
                StatusCode::BAD_REQUEST,
                "The request was malformed or unable to be completed",
            ),
            // Self::Session(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Session storage failed"),
            // Self::Auth(_) => (StatusCode::FORBIDDEN, "You don't have permission to do that"),
        };

        (status, client_message).into_response()
    }
}
