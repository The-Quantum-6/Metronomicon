use crate::error::{AppError, RequestError};
use crate::state::{self, AppState};
use crate::{extractors::report::ReportCommandExtractor, middleware::perms::perm_middleware};
use axum::middleware;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use serde::Serialize;
pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/reports", get(list_reports))
        .route("/reports", post(handle_command))
        .route("/reports/{id}", get(query_handler))
}

pub async fn handle_command(
    State(state): State<AppState>,
    ReportCommandExtractor(metadata, command): ReportCommandExtractor,
) -> Result<(), AppError> {
    let issue_id = command.id();

    state
        .cqrs
        .report
        .execute_with_metadata(&issue_id.to_string(), command, metadata)
        .await
        .map_err(|_e| AppError::BadRequest(RequestError::NonExsistant("Report")))?;

    Ok(())
}

pub async fn list_reports(
    State(state): State<AppState>,
) -> Result<Json<Vec<ReportListItem>>, AppError> {
    let reports = sqlx::query_as::<_, ReportListItem>(
        r#"
        SELECT aggregate_id, target, title, description, contact_email, status
        FROM report_detail_list_view
        ORDER BY aggregate_id
        "#,
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(reports))
}

pub async fn query_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ReportListItem>, AppError> {
    let report = sqlx::query_as::<_, ReportListItem>(
        r#"
        SELECT aggregate_id, target, title, description, contact_email, status
        FROM report_detail_list_view
        WHERE aggregate_id = $1
        "#,
    )
    .bind(&id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::BadRequest(RequestError::NonExsistant("Report")))?;

    Ok(Json(report))
}
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ReportListItem {
    pub aggregate_id: String,
    pub target: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub contact_email: Option<String>,
    pub status: Option<String>,
}
