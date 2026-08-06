use axum::body::{Body, Bytes};
use axum::extract::FromRequest;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use std::collections::HashMap;
use uuid::Uuid;

use crate::aggregates::project_idea::command::ProjectIdeaCommand;

pub struct ProjectIdeaCommandExtractor(pub HashMap<String, String>, pub ProjectIdeaCommand);

impl<S> FromRequest<S> for ProjectIdeaCommandExtractor
where
    S: Send + Sync,
{
    type Rejection = ProjectIdeaCommandExtractionError;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let mut metadata = HashMap::default();
        metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());

        let body = Bytes::from_request(req, state).await?;

        let mut json_value: serde_json::Value = serde_json::from_slice(&body)?;

        if let Some(create_obj) = json_value.get_mut("Create").and_then(|v| v.as_object_mut()) {
            create_obj.insert(
                "idea_id".to_string(),
                serde_json::Value::String(Uuid::new_v4().to_string()),
            );
        }

        let command: ProjectIdeaCommand = serde_json::from_value(json_value)?;
        Ok(Self(metadata, command))
    }
}

pub struct ProjectIdeaCommandExtractionError;

impl IntoResponse for ProjectIdeaCommandExtractionError {
    fn into_response(self) -> Response {
        (
            StatusCode::BAD_REQUEST,
            "command could not be read".to_string(),
        )
            .into_response()
    }
}

impl From<axum::extract::rejection::BytesRejection> for ProjectIdeaCommandExtractionError {
    fn from(_: axum::extract::rejection::BytesRejection) -> Self {
        Self
    }
}

impl From<serde_json::Error> for ProjectIdeaCommandExtractionError {
    fn from(_: serde_json::Error) -> Self {
        Self
    }
}
