use crate::{
    auth::jwt::generate_access, models::claims::AccessClaim, repositories::user::get_user_by_id,
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
pub struct AuthUser(pub AccessClaim);

pub async fn jwt_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    session: Session,
    mut req: Request,
    next: Next,
) -> Response {
    let cookie_header = match headers.get("cookie") {
        Some(h) => h,
        None => return Redirect::to("/login/google").into_response(),
    };

    println!("Cookie header: {:?}", cookie_header);

    let token_str = match cookie_header
        .to_str()
        .ok()
        .and_then(|s| s.split(';').find(|c| c.trim().starts_with("access_token=")))
    {
        Some(t) => t.trim().trim_start_matches("access_token=").to_string(),
        None => return Redirect::to("/login/google").into_response(),
    };
    print!("Token string: {:?}", token_str);

    let mut validation = Validation::default();
    validation.set_required_spec_claims(&["exp", "sub"]);
    validation.leeway = 0;

    match decode::<AccessClaim>(&token_str, &state.jwt_decode, &validation) {
        Ok(t) => {
            req.extensions_mut().insert(t.claims);
            next.run(req).await
        }
        Err(e) => match e.kind() {
            ErrorKind::ExpiredSignature => match try_refresh(&state, &session).await {
                Some((new_token, claims)) => {
                    let cookie = format!(
                        "access_token={}; HttpOnly; SameSite=Lax; Path=/; Max-Age=900",
                        new_token
                    );
                    req.extensions_mut().insert(claims);
                    let mut response = next.run(req).await;
                    response
                        .headers_mut()
                        .insert(header::SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());
                    response
                }
                None => Redirect::to("/login/google").into_response(),
            },
            _ => StatusCode::UNAUTHORIZED.into_response(),
        },
    }
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

async fn try_refresh(state: &AppState, session: &Session) -> Option<(String, AccessClaim)> {
    let _refresh_token: String = session.get("refresh_token").await.ok()??;
    let user_id: Uuid = session.get("user_id").await.ok()??;
    let user = get_user_by_id(&state.pool, user_id).await.ok()??;

    let new_token = generate_access(&state.jwt_encode, user.role, user.id.to_string()).ok()?;

    let new_refresh = Uuid::new_v4().to_string();
    session.insert("refresh_token", new_refresh).await.ok()?;

    let mut validation = Validation::default();
    validation.set_required_spec_claims(&["exp", "sub"]);
    let claims = jsonwebtoken::decode::<AccessClaim>(&new_token, &state.jwt_decode, &validation)
        .ok()?
        .claims;
    Some((new_token, claims))
}
