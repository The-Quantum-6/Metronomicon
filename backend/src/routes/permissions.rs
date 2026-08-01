use crate::{
    middleware::{jwt::AuthUser, perms::transfer_perm_middleware}, models::{permissions::Permissions, user::UserRole}, repositories::permissions::{
        get_all_users_permissions, get_user_permissions, update_permissions,
    }, state::AppState,
};
use axum::{
    Json, Router, extract::{Query, State}, http::StatusCode, middleware::from_fn_with_state, routing::{get, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


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