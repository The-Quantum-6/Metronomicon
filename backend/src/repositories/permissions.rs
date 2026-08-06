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
    let default_perms =
        (Permissions::READ | Permissions::SUGGEST_TEXT | Permissions::SUGGEST_FILE).bits();

    sqlx::query!(
        r#"
        INSERT INTO permissions (user_id, perms, resource_id) 
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id, resource_id) DO NOTHING
        "#,
        user_id,
        default_perms,
        resource_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub struct UserWithPerms {
    pub user_id: Uuid,
    pub name: String,
    pub perms: Permissions,
}

pub async fn get_all_users_permissions(
    pool: &PgPool,
    resource_id: &str,
) -> Result<Vec<UserWithPerms>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT u.id, u.name, COALESCE(p.perms, 0) as perms
        FROM users u
        LEFT JOIN permissions p ON u.id = p.user_id AND p.resource_id = $1
        "#,
        resource_id
    )
    .fetch_all(pool)
    .await?;

    let list = rows
        .into_iter()
        .map(|row| UserWithPerms {
            user_id: row.id,
            name: row.name,
            perms: Permissions::from_bits_truncate(row.perms.unwrap_or(0)),
        })
    .collect();

    Ok(list)
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
