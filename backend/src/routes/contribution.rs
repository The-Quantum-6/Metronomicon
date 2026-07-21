use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use serde::{Deserialize, Serialize};

use crate::extractors::contribution::ContributionCommandExtractor;
use crate::middleware::perms::perm_middleware;
use crate::state::AppState;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new().route(
        "/contributions",
        get(list_contributions).post(handle_command),
    )
    .route_layer(middleware::from_fn_with_state(state, perm_middleware))
}

pub async fn handle_command(
    State(state): State<AppState>,
    ContributionCommandExtractor(metadata, command): ContributionCommandExtractor,
) -> Response {
    let contribution_id = command.id();

    match state
        .cqrs
        .contribution
        .execute_with_metadata(&contribution_id.to_string(), command, metadata)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            println!("Error: {e:#?}\n");
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
    }
}

pub async fn list_contributions(
    State(state): State<AppState>,
) -> Result<Json<Vec<ContributionDTO>>, StatusCode> {
    Ok(Json(
        sqlx::query_as!(
            ContributionDTO,
            r#"
            SELECT aggregate_id, course_id, contribution, status
            FROM contribution_list_view
            ORDER BY aggregate_id
            "#
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}

#[derive(Serialize, Deserialize)]
pub struct ContributionDTO {
    aggregate_id: String,
    course_id: String,
    contribution: serde_json::Value,
    status: String,
}
