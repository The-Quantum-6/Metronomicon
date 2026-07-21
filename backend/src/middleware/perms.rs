use crate::{
    models::{claims::AccessClaim, permissions::Permissions}, repositories::permissions::get_user_permissions, state::AppState,
};
use axum::{
    body::{Body, to_bytes},
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::collections::HashMap;
use std::sync::OnceLock;
use uuid::Uuid;

const PERMISSION_ENTRIES: &[(&str, Permissions)] = &[
    // Faq
    ("FaqCreate", Permissions::WRITE_TEXT),
    ("FaqUpdate", Permissions::WRITE_TEXT),
    ("FaqDelete", Permissions::WRITE_TEXT),
    ("FaqSetOfficial", Permissions::PAGE_ADMIN),
    // Link
    ("LinkCreate", Permissions::WRITE_TEXT),
    ("LinkUpdate", Permissions::WRITE_TEXT),
    ("LinkDelete", Permissions::WRITE_TEXT),
    ("LinkSetOfficial", Permissions::PAGE_ADMIN),
    // ProjectIdea
    ("ProjectIdeaCreate", Permissions::WRITE_TEXT),
    ("ProjectIdeaUpdate", Permissions::WRITE_TEXT),
    ("ProjectIdeaDelete", Permissions::WRITE_TEXT),
    // Resource
    ("ResourceCreate", Permissions::WRITE_FILE),
    ("ResourceUpdate", Permissions::WRITE_FILE),
    ("ResourceDelete", Permissions::WRITE_FILE),
    ("ResourceSetOfficial", Permissions::PAGE_ADMIN),
    // Report
    ("ReportCreate", Permissions::SUGGEST_TEXT),
    ("ReportResolveReport", Permissions::PAGE_ADMIN),
    ("ReportReopenReport", Permissions::PAGE_ADMIN),
    // Contribution
    ("ContributionPropose_Text", Permissions::SUGGEST_TEXT),
    ("ContributionPropose_File", Permissions::SUGGEST_FILE),
    ("ContributionModerate_Text", Permissions::MODERATE_TEXT),
    ("ContributionModerate_File", Permissions::MODERATE_FILE),
];

fn permission_map() -> &'static HashMap<&'static str, Permissions> {
    static MAP: OnceLock<HashMap<&'static str, Permissions>> = OnceLock::new();
    MAP.get_or_init(|| PERMISSION_ENTRIES.iter().cloned().collect())
}

fn aggregate_from_path(path: &str) -> &str {
    path.trim_start_matches('/').split('/').next().unwrap_or("")
}

fn capitalize_singular(s: &str) -> String {
    let singular = s.trim_end_matches('s');
    let mut c = singular.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
    }
}

pub async fn perm_middleware(State(state): State<AppState>, req: Request, next: Next) -> Response {
    let path = req.uri().path().to_string();
    let aggregate = capitalize_singular(aggregate_from_path(&path));

    let (parts, body) = req.into_parts();
    let bytes = match to_bytes(body, usize::MAX).await {
        Ok(b) => b,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or_default();

    let command_key = json
        .as_object()
        .and_then(|o| o.keys().next().cloned())
        .unwrap_or_default();

    let lookup_key = format!("{}{}", aggregate, command_key);

    let required = match permission_map().get(lookup_key.as_str()) {
        Some(p) => *p,
        None => return StatusCode::FORBIDDEN.into_response(),
    };

    let claims = match parts.extensions.get::<AccessClaim>().cloned() {
        Some(c) => c,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };
    
    let course_id = json
        .as_object()
        .and_then(|o| o.values().next())
        .and_then(|v| v.get("course_id"))
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let perms = match get_user_permissions(&state.pool, user_id, &course_id).await {
        Ok(p) => p,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if !perms.contains(required) {
        return StatusCode::FORBIDDEN.into_response();
    }

    let req = Request::from_parts(parts, Body::from(bytes));
    next.run(req).await
}
