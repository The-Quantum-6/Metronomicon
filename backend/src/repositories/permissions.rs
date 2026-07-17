use crate::models::permissions::Permissions;
use sqlx::PgPool;
use uuid::Uuid;
const GROUP_ID: &str = "default";

pub async fn get_user_permissions(pool: &PgPool, user_id: Uuid) -> Result<Permissions, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT perms FROM permissions WHERE user_id=$1 AND group_id=$2",
        user_id,
        GROUP_ID,
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
) -> Result<(), sqlx::Error> {

    sqlx::query!(
        "INSERT INTO permissions(user_id, perms, group_id) VALUES ($1, $2, $3)",
        user_id,
        Permissions::READ.bits() as i32,
        GROUP_ID,
    )
    .execute(pool)
    .await
    .map(|_| ())
}

pub async fn update_permissions(pool: &PgPool, id: Uuid, permissions: Permissions) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE permissions SET perms=$1 WHERE user_id=$2 AND group_id=$3",
        permissions.bits() as i32,
        id,
        GROUP_ID,
    )
    .execute(pool)
    .await
    .map(|_| ())
}

pub async fn delete_permissions(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM permissions WHERE user_id=$1 AND group_id=$2",
        user_id,
        GROUP_ID,
    )
    .execute(pool)
    .await
    .map(|_| ())

}
