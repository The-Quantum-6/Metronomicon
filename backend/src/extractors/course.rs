use axum::body::{Body, Bytes};
use axum::extract::FromRequest;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use uuid::Uuid;
use std::collections::HashMap;

use crate::aggregates::course::command::CourseCommand;

pub struct CourseCommandExtractor(pub HashMap<String, String>, pub CourseCommand);

// const USER_AGENT_HDR: &str = "User-Agent";

impl<S> FromRequest<S> for CourseCommandExtractor
where
    S: Send + Sync,
{
    type Rejection = CourseCommandExtractionError;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let mut metadata = HashMap::default();
        // Insert timestamp
        metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());
        // metadata.insert("uri".to_string(), req.uri().to_string());
        // if let Some(user_agent) = req.headers().get(USER_AGENT_HDR) {
        //     if let Ok(value) = user_agent.to_str() {
        //         metadata.insert(USER_AGENT_HDR.to_string(), value.to_string());
        //     }
        // }

        // TODO: This may be the place attach the user id of the user that is issuing a command.
        // We may want to create a higher level extractor that uses this one to collect those functionalities
        
        let body = Bytes::from_request(req, state).await?;

        let mut json_value: serde_json::Value = serde_json::from_slice(&body)?;

        // Only inject course_id when the command is a Create
        if let Some(create_obj) = json_value
            .get_mut("Create")
            .and_then(|v| v.as_object_mut())
        {
            create_obj.insert(
                "course_id".to_string(),
                serde_json::Value::String(Uuid::new_v4().to_string()),
            );
        }

        let command: CourseCommand = serde_json::from_value(json_value)?;
        Ok(Self(metadata, command))
    }
}

pub struct CourseCommandExtractionError;

impl IntoResponse for CourseCommandExtractionError {
    fn into_response(self) -> Response {
        (
            StatusCode::BAD_REQUEST,
            "command could not be read".to_string(),
        )
            .into_response()
    }
}

impl From<axum::extract::rejection::BytesRejection> for CourseCommandExtractionError {
    fn from(_: axum::extract::rejection::BytesRejection) -> Self {
        Self
    }
}

impl From<serde_json::Error> for CourseCommandExtractionError {
    fn from(_: serde_json::Error) -> Self {
        Self
    }
}
