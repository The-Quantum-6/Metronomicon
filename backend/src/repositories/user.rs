use crate::models::user::{User, UserRole};
use sqlx::PgPool;
use uuid::Uuid;

/// Fetches all users from the database.
///
/// Returns an empty `Vec` (not an error) if no users exist.
pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"SELECT id, sub, name, role AS "role: UserRole", email FROM users"#
    )
    .fetch_all(pool)
    .await
}

/// Inserts a new user into the database.
///
/// `email` is optional; pass `None` if the user has no email on record.
/// Does not check for an existing user with the same `sub` — duplicates
/// are allowed unless constrained at the database level.
pub async fn create_user(
    pool: &PgPool,
    sub: Uuid,
    name: String,
    email: Option<String>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO users(sub, name, email) VALUES ($1, $2, $3)",
        sub,
        name,
        email
    )
    .execute(pool)
    .await
    .map(|_| ())
}

/// Fetches a single user by their internal database `id`.
///
/// Returns `Ok(None)` if no user with that `id` exists.
pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"SELECT id, sub, name, role AS "role: UserRole", email FROM users WHERE id=$1"#,
        id
    )
    .fetch_optional(pool)
    .await
}

/// Fetches a single user by their `sub` (the external/auth-provider identifier).
///
/// Returns `Ok(None)` if no user with that `sub` exists.
pub async fn get_user_by_sub(pool: &PgPool, sub: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"SELECT id, sub, name, role AS "role: UserRole", email FROM users WHERE sub=$1"#,
        sub
    )
    .fetch_optional(pool)
    .await
}

/// Deletes a user by their internal database `id`.
///
/// Succeeds even if no user with that `id` exists (deleting zero rows
/// is not treated as an error).
pub async fn delete_user(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM users WHERE id=$1", id)
        .execute(pool)
        .await
        .map(|_| ())
}