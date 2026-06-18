use crate::models::user::{User, UserRole};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"SELECT id, sub, name, role AS "role: UserRole", email FROM users"#
    )
    .fetch_all(pool)
    .await
}

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

pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"SELECT id, sub, name, role AS "role: UserRole", email FROM users WHERE id=$1"#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn get_user_by_sub(pool: &PgPool, sub: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"SELECT id, sub, name, role AS "role: UserRole", email FROM users WHERE sub=$1"#,
        sub
    )
    .fetch_optional(pool)
    .await
}

pub async fn delete_user(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM users WHERE id=$1", id)
        .execute(pool)
        .await
        .map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[sqlx::test]
    async fn get_users_returns_correct_number(pool: PgPool) -> sqlx::Result<()> {
        let users = get_users(&pool).await?;

        assert!(users.is_empty());

        let name = "John".to_string();
        let sub = Uuid::new_v4();
        let email = None::<String>;

        create_user(&pool, sub, name.clone(), email.clone()).await?;
        create_user(&pool, sub, name.clone(), email.clone()).await?;
        create_user(&pool, sub, name.clone(), email.clone()).await?;

        let users = get_users(&pool).await?;

        assert_eq!(users.len(), 3);

        Ok(())
    }
    #[sqlx::test]
    async fn get_specific_user(pool: PgPool) -> sqlx::Result<()> {
        let users = get_users(&pool).await?;

        assert!(users.is_empty());

        let name = "John".to_string();
        let sub = Uuid::new_v4();
        let email = None::<String>;

        create_user(&pool, sub, name.clone(), email.clone()).await?;

        let user = get_user_by_sub(&pool, sub).await?;

        assert!(user.is_some());
        assert_eq!(user.unwrap().name, name);

        Ok(())
    }
    #[sqlx::test]
    async fn create_user_stores_correct_fields(pool: PgPool) -> sqlx::Result<()> {
        let sub = Uuid::new_v4();
        let name = "Jane".to_string();
        let email = Some("jane@example.com".to_string());

        create_user(&pool, sub, name.clone(), email.clone()).await?;

        let user = get_user_by_sub(&pool, sub)
            .await?
            .expect("user should exist");
        assert_eq!(user.sub, sub);
        assert_eq!(user.name, name);
        assert_eq!(user.email, email);
        Ok(())
    }

    #[sqlx::test]
    async fn create_user_with_no_email(pool: PgPool) -> sqlx::Result<()> {
        let sub = Uuid::new_v4();
        let name = "NoEmail".to_string();

        create_user(&pool, sub, name.clone(), None).await?;

        let user = get_user_by_sub(&pool, sub)
            .await?
            .expect("user should exist");
        assert_eq!(user.email, None);
        Ok(())
    }

    #[sqlx::test]
    async fn get_user_by_id_returns_correct_user(pool: PgPool) -> sqlx::Result<()> {
        let sub = Uuid::new_v4();
        let name = "Found".to_string();
        create_user(&pool, sub, name.clone(), None).await?;

        let created = get_user_by_sub(&pool, sub).await?.expect("should exist");
        let fetched = get_user_by_id(&pool, created.id)
            .await?
            .expect("should exist");

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.name, name);
        Ok(())
    }

    #[sqlx::test]
    async fn get_user_by_id_returns_none_for_unknown_id(pool: PgPool) -> sqlx::Result<()> {
        let result = get_user_by_id(&pool, Uuid::new_v4()).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[sqlx::test]
    async fn get_user_by_sub_returns_none_for_unknown_sub(pool: PgPool) -> sqlx::Result<()> {
        let result = get_user_by_sub(&pool, Uuid::new_v4()).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[sqlx::test]
    async fn delete_user_removes_user(pool: PgPool) -> sqlx::Result<()> {
        let sub = Uuid::new_v4();
        create_user(&pool, sub, "ToDelete".to_string(), None).await?;

        let user = get_user_by_sub(&pool, sub).await?.expect("should exist");
        delete_user(&pool, user.id).await?;

        let result = get_user_by_id(&pool, user.id).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[sqlx::test]
    async fn delete_user_nonexistent_id_does_not_error(pool: PgPool) -> sqlx::Result<()> {
        // Deleting a row that doesn't exist should succeed silently (0 rows affected),
        // not return an error.
        let result = delete_user(&pool, Uuid::new_v4()).await;
        assert!(result.is_ok());
        Ok(())
    }

    #[sqlx::test]
    async fn get_users_returns_multiple_distinct_users(pool: PgPool) -> sqlx::Result<()> {
        create_user(&pool, Uuid::new_v4(), "A".to_string(), None).await?;
        create_user(&pool, Uuid::new_v4(), "B".to_string(), None).await?;
        create_user(&pool, Uuid::new_v4(), "C".to_string(), None).await?;

        let users = get_users(&pool).await?;
        assert_eq!(users.len(), 3);
        Ok(())
    }
}
