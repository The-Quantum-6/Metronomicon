use crate::extractors::report::ReportCommandExtractor;
use crate::state::AppState;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::Serialize;

pub fn router() -> Router<AppState> {
    Router::new()
       .route("/reports", get(list_reports))
       .route("/reports", post(handle_command))
}

pub async fn handle_command(
    State(state): State<AppState>,
    ReportCommandExtractor(metadata, command): ReportCommandExtractor,
) -> Response {
    let issue_id = command.id();

    match state
        .cqrs
        .report
        .execute_with_metadata(&issue_id.to_string(), command, metadata)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),

        Err(e) => {
            println!("Error: {e:#?}");
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
    }
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ReportListItem {
    pub aggregate_id: String,
    pub target: Option<String>,
    pub description: Option<String>,
    pub contact_email: Option<String>,
    pub status: Option<String>,
}

pub async fn list_reports(State(state): State<AppState>) -> Response {
    let result = sqlx::query_as::<_, ReportListItem>(
        "SELECT aggregate_id, target, description, contact_email, status
         FROM report_list_view
         ORDER BY aggregate_id",
    )
    .fetch_all(&state.pool)
    .await;

    match result {
        Ok(reports) => Json(reports).into_response(),
        Err(e) => {
            println!("list_reports error: {e:#?}");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn query_handler(
    State(_state): State<AppState>,
    Path(_id): Path<String>,
) -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}