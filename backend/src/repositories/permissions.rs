use crate::models::permissions::Permissions;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_user_permissions(
    pool: &PgPool,
    user_id: Uuid,
    resource_id: &str,
) -> Result<Permissions, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT perms FROM permissions WHERE user_id=$1 AND resource_id=$2",
        user_id,
        resource_id,
    )
    .fetch_optional(pool)
    .await?;

    if let Some(row) = row {
        Ok(Permissions::from_bits_truncate(row.perms as i32))
    } else {
        Ok(Permissions::empty())
    }
}

pub async fn default_permissions(
    pool: &PgPool,
    user_id: Uuid,
    resource_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO permissions(user_id, perms, resource_id) VALUES ($1, $2, $3)",
        user_id,
        Permissions::READ.bits() as i32,
        resource_id,
    )
    .execute(pool)
    .await
    .map(|_| ())
}

pub async fn update_permissions(
    pool: &PgPool,
    user_id: Uuid,
    resource_id: &str,
    permissions: Permissions,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE permissions SET perms=$1 WHERE user_id=$2 AND resource_id=$3",
        permissions.bits() as i32,
        user_id,
        resource_id,
    )
    .execute(pool)
    .await
    .map(|_| ())
}

pub async fn delete_permissions(
    pool: &PgPool,
    user_id: Uuid,
    resource_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM permissions WHERE user_id=$1 AND resource_id=$2",
        user_id,
        resource_id,
    )
    .execute(pool)
    .await
    .map(|_| ())
}