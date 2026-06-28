use axum::body::{Body, Bytes};
use axum::extract::FromRequest;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use uuid::Uuid;
use std::collections::HashMap;

use crate::aggregates::link::command::LinkCommand;

pub struct LinkCommandExtractor(pub HashMap<String, String>, pub LinkCommand);

// const USER_AGENT_HDR: &str = "User-Agent";

impl<S> FromRequest<S> for LinkCommandExtractor
where
    S: Send + Sync,
{
    type Rejection = LinkCommandExtractionError;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let mut metadata = HashMap::default();
        metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());

        // metadata.insert("uri".to_string(), req.uri().to_string());
        // if let Some(user_agent) = req.headers().get(USER_AGENT_HDR) {
        //     if let Ok(value) = user_agent.to_str() {
        //         metadata.insert(USER_AGENT_HDR.to_string(), value.to_string());
        //     }
        // }

        // TODO: This may be the place attach the user id of the user that is issuing a command.
        // We may want to create a higher level extractor that uses this one to collect those functionalities.

        let body = Bytes::from_request(req, state).await?;

        let mut json_value: serde_json::Value = serde_json::from_slice(&body)?;

        if let Some(create_obj) = json_value
            .get_mut("Create")
            .and_then(|v| v.as_object_mut())
        {
            create_obj.insert(
                "link_id".to_string(),
                serde_json::Value::String(Uuid::new_v4().to_string()),
            );
        }

        let command: LinkCommand = serde_json::from_value(json_value)?;
        Ok(Self(metadata, command))
    }
}

pub struct LinkCommandExtractionError;

impl IntoResponse for LinkCommandExtractionError {
    fn into_response(self) -> Response {
        (
            StatusCode::BAD_REQUEST,
            "command could not be read".to_string(),
        )
            .into_response()
    }
}

impl From<axum::extract::rejection::BytesRejection> for LinkCommandExtractionError {
    fn from(_: axum::extract::rejection::BytesRejection) -> Self {
        Self
    }
}

impl From<serde_json::Error> for LinkCommandExtractionError {
    fn from(_: serde_json::Error) -> Self {
        Self
    }
}
