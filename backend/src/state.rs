//! Shared application state.

use crate::storage::Storage;
use axum::extract::FromRef;
use sqlx::PgPool;

/// Everything handlers might need, bundled into one value given to the router.
///
/// The `FromRef` impls below let a handler ask for just one piece — for example
/// `State<PgPool>` for the database or `State<Storage>` for object storage —
/// without having to take the whole `AppState`.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub storage: Storage,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppState> for Storage {
    fn from_ref(state: &AppState) -> Self {
        state.storage.clone()
    }
}
