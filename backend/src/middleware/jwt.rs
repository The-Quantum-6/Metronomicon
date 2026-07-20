use crate::{models::claims::AccessClaim, state::AppState};
use axum::{
    extract::{FromRequestParts, Request, State},
    http::{HeaderMap, StatusCode, request::Parts},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use jsonwebtoken::{Validation, decode, errors::ErrorKind};
pub struct AuthUser(pub AccessClaim);

pub async fn jwt_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Response {
    let cookie_header = match headers.get("cookie") {
        Some(h) => h,
        None => return Redirect::to("/login").into_response(),
    };

    let token_str = match cookie_header
        .to_str()
        .ok()
        .and_then(|s| s.split(';').find(|c| c.trim().starts_with("access_token=")))
    {
        Some(t) => t.trim().trim_start_matches("access_token=").to_string(),
        None => return Redirect::to("/login").into_response(),
    };

    let mut validation = Validation::default();
    validation.set_required_spec_claims(&["exp", "sub"]);

    let token = match decode::<AccessClaim>(&token_str, &state.jwt_decode, &validation) {
        Ok(t) => t,
        Err(e) => match e.kind() {
            ErrorKind::ExpiredSignature => {
                // TODO: refresh token
                return Redirect::to("/login").into_response();
            }
            _ => return StatusCode::UNAUTHORIZED.into_response(),
        },
    };

    req.extensions_mut().insert(token.claims);
    next.run(req).await
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AccessClaim>()
            .cloned()
            .map(AuthUser)
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}
