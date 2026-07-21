use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    extract::State,
};
use uuid::Uuid;
use crate::{repositories::permissions::get_user_permissions, state::AppState};
use crate::models::claims::AccessClaim;
use crate::models::permissions::Permissions;

pub async fn check_perm(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Siden vi bruker Axum sin state-uttrekking, må vi hente rettigheten 
    // som kreves fra forespørselens extensions (vi legger den inn i routeren)
    let required_perm = req.extensions()
        .get::<Permissions>()
        .cloned()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let _auth_user = req.extensions()
        .get::<AccessClaim>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    //let user_id = Uuid::parse_str(&auth_user.sub)
      //  .map_err(|_| StatusCode::BAD_REQUEST)?;
    // I stedet for å hente fra sesjonen, bare hardcoder du UUID-en din midlertidig:
    let user_id: Uuid = Uuid::parse_str("019f802f-0e14-70be-97ac-9f64be7dda66").unwrap();
    let resource_id = req.extensions()
        .get::<String>()
        .cloned()
        .unwrap_or_default();

    let user_perms = get_user_permissions(&state.pool, user_id, &resource_id)
        .await
        .unwrap_or(Permissions::empty());

    if !user_perms.contains(required_perm) { 
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(req).await)
}