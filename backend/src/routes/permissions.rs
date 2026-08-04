use crate::{
    auth::jwt::generate_perms, middleware::{jwt::AuthUser, perms::transfer_perm_middleware}, models::{claims::PermsClaim, permissions::Permissions, user::{self, UserRole}}, repositories::{permissions::{
        get_all_users_permissions, get_user_permissions, update_permissions,
    }, user::get_user_by_sub}, state::AppState,
};
use aws_config::default_provider::token;
use axum::{
    Json, Router, extract::{Query, State}, http::{HeaderValue, StatusCode}, middleware::from_fn_with_state, response::{IntoResponse, Response}, routing::{get, post},
};
use reqwest::header;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn protected_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/permissions/token", post(perm_token))
}

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/permissions", get(get_perms))
        .route("/permissions", post(update_perms))
        .route("/permissions/all", get(get_all_perms))
        .route_layer(from_fn_with_state(
            state,
            transfer_perm_middleware,
        ))
}
#[derive(Deserialize)]
pub struct GetPermsQuery {
    pub user_id: Uuid,
    pub course_id: String,
}

#[derive(Deserialize)]
pub struct PermTokenQuery {
    pub course_id: String,
}

#[derive(Deserialize)]
pub struct GetAllPermsQuery {
    pub course_id: String,
}

#[derive(Serialize)]
pub struct UserPermsResponse {
    pub user_id: Uuid,
    pub name: String,
    pub perms: i32,
}

pub async fn get_all_perms(
    State(state): State<AppState>,
    AuthUser(_claims): AuthUser,
    Query(query): Query<GetAllPermsQuery>,
) -> Result<Json<Vec<UserPermsResponse>>, StatusCode> {
    let perms_list = get_all_users_permissions(&state.pool, &query.course_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = perms_list
        .into_iter()
        .map(|u| UserPermsResponse {
            user_id: u.user_id,
            name: u.name,
            perms: u.perms.bits(),
        })
        .collect();

    Ok(Json(response))
}

pub async fn get_perms(
    State(state): State<AppState>,
    AuthUser(_claims): AuthUser,
    Query(query): Query<GetPermsQuery>,
) -> Result<Json<i32>, StatusCode> {
    let perms = get_user_permissions(&state.pool, query.user_id, &query.course_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(perms.bits()))
}

#[derive(Deserialize)]
pub struct UpdatePermsPayload {
    pub user_id: Uuid,
    pub course_id: String,
    pub permissions: i32,
}

pub async fn update_perms(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Json(payload): Json<UpdatePermsPayload>,
) -> Result<StatusCode, StatusCode> {
    let is_admin = claims.role == UserRole::Admin;

    if !is_admin {
        let current_target_perms = get_user_permissions(&state.pool, payload.user_id, &payload.course_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let requested_perms = Permissions::from_bits_truncate(payload.permissions);

        let had_transfer = current_target_perms.contains(Permissions::TRANSFER_PERMS);
        let will_have_transfer = requested_perms.contains(Permissions::TRANSFER_PERMS);

        if had_transfer != will_have_transfer {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    update_permissions(
        &state.pool,
        payload.user_id,
        &payload.course_id,
        Permissions::from_bits_truncate(payload.permissions),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}



pub async fn perm_token(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser, 
    Query(query): Query<PermTokenQuery>,
) -> Result<Response, StatusCode> {

    let userid = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let perms = get_user_permissions(&state.pool, userid, &query.course_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let new_token = generate_perms(
        &state.jwt_encode,
        query.course_id.clone(),
        perms.bits() as i32,
        claims.sub,
        claims.role
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cookie_val = format!(
        "{}={}; Path=/courses/{}; SameSite=Lax; Max-Age=3600",
        query.course_id,
        new_token,
        query.course_id
    );

    let mut response = Json(new_token).into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie_val).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );

    Ok(response)
}