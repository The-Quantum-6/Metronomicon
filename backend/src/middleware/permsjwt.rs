use crate::{
    auth::jwt::generate_access, models::claims::PermsClaim, repositories::user::get_user_by_id,
    state::AppState,
};
use axum::{
    extract::{FromRequestParts, Request, State},
    http::{HeaderMap, HeaderValue, StatusCode, request::Parts},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use jsonwebtoken::{Validation, decode, errors::ErrorKind};
use reqwest::header;
use tower_sessions::{Session, session};
use uuid::Uuid;
pub struct PermsUser(pub PermsClaim);

pub async fn perms_jwt_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    session: Session,
    mut req: Request,
    next: Next,
) -> Response {
    let cookie_header = match headers.get("cookie") {
        Some(h) => h,
        None => return StatusCode::UNAUTHORIZED.into_response(),
       };

    let token_str = match cookie_header
        .to_str()
        .ok()
        .and_then(|s| s.split(';').find(|c| c.trim().starts_with("perms_token=")))
    {
        Some(t) => t.trim().trim_start_matches("perms_token=").to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let mut validation = Validation::default();
    validation.set_required_spec_claims(&["exp", "sub"]);
    validation.leeway = 0;

    match decode::<PermsClaim>(&token_str, &state.jwt_decode, &validation) {
        Ok(t) => {
            req.extensions_mut().insert(t.claims);
            next.run(req).await
        }
        Err(_) => {
            StatusCode::UNAUTHORIZED.into_response()
        }
    }
}

impl<S> FromRequestParts<S> for PermsUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<PermsClaim>()
            .cloned()
            .map(PermsUser)
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}