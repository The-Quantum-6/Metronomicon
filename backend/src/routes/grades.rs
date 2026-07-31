use axum::{
    extract::{Path, State},
    response::Json,
    routing::get,
    Router,
};
use serde_json::Value;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/grades/{course}", get(get_grades))
}

pub async fn get_grades(
    State(state): State<AppState>,
    Path(course): Path<String>,
) -> Result<Json<Value>, String> {
    let url = format!(
        "https://api.karakterweb.no/v1/courses/oslomet/{}?include=grades&semester=both",
        course
    );

    let response = state
        .http_client
        .get(url)
        .header("X-Client-Key", &state.karakterweb_api_key)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json = response
        .json::<Value>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(json))
}