use crate::models::course::Course;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_courses(pool: &PgPool) -> Result<Vec<Course>, sqlx::Error> {
    sqlx::query_as!(Course, r#"SELECT id, content, name, code FROM courses"#)
        .fetch_all(pool)
        .await
}

pub async fn create_course(
    pool: &PgPool,
    name: String,
    content: Option<String>,
    code: String,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO courses(name, content, code) VALUES ($1, $2, $3)",
        name,
        content,
        code
    )
    .execute(pool)
    .await
    .map(|_| ())
}

pub async fn get_course_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Course>, sqlx::Error> {
    sqlx::query_as!(
        Course,
        r#"SELECT id, content, name, code FROM courses WHERE id=$1"#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn get_course_by_code(pool: &PgPool, code: &str) -> Result<Option<Course>, sqlx::Error> {
    sqlx::query_as!(
        Course,
        r#"SELECT id, content, name, code FROM courses WHERE code=$1"#,
        code
    )
    .fetch_optional(pool)
    .await
}

pub async fn update_course(
    pool: &PgPool,
    id: Uuid,
    name: String,
    content: Option<String>,
    code: String,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE courses SET name=$1, content=$2, code=$3 WHERE id=$4",
        name,
        content,
        code,
        id
    )
    .execute(pool)
    .await
    .map(|_| ())
}

pub async fn delete_course(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM courses WHERE id=$1", id)
        .execute(pool)
        .await
        .map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn get_courses_returns_correct_number(pool: PgPool) -> sqlx::Result<()> {
        let courses = get_courses(&pool).await?;
        assert!(courses.is_empty());

        let name = "John".to_string();
        let content = None::<String>;
        let code = "C101".to_string();

        create_course(&pool, name.clone(), content.clone(), code.clone()).await?;
        create_course(&pool, name.clone(), content.clone(), code.clone()).await?;
        create_course(&pool, name.clone(), content.clone(), code.clone()).await?;

        let courses = get_courses(&pool).await?;
        assert_eq!(courses.len(), 3);
        Ok(())
    }

    #[sqlx::test]
    async fn get_specific_course(pool: PgPool) -> sqlx::Result<()> {
        let courses = get_courses(&pool).await?;
        assert!(courses.is_empty());

        let name = "John".to_string();
        let content = None::<String>;
        let code = "C101".to_string();

        create_course(&pool, name.clone(), content.clone(), code.clone()).await?;

        let course = get_course_by_code(&pool, &code).await?;
        assert!(course.is_some());
        assert_eq!(course.unwrap().code, code);
        Ok(())
    }

    #[sqlx::test]
    async fn create_course_stores_correct_fields(pool: PgPool) -> sqlx::Result<()> {
        let name = "Jane".to_string();
        let content = Some("Course content".to_string());
        let code = "C102".to_string();

        create_course(&pool, name.clone(), content.clone(), code.clone()).await?;

        let course = get_course_by_code(&pool, &code)
            .await?
            .expect("course should exist");
        assert_eq!(course.code, code);
        assert_eq!(course.content, content);
        assert_eq!(course.name, name);
        Ok(())
    }

    #[sqlx::test]
    async fn get_course_by_id_returns_correct_course(pool: PgPool) -> sqlx::Result<()> {
        let name = "Found".to_string();
        let content = Some("Course content".to_string());
        let code = "C104".to_string();
        create_course(&pool, name.clone(), content.clone(), code.clone()).await?;

        let course = get_course_by_code(&pool, &code)
            .await?
            .expect("course should exist");
        let fetched = get_course_by_id(&pool, course.id)
            .await?
            .expect("course should exist");

        assert_eq!(fetched.id, course.id);
        assert_eq!(fetched.name, name);
        assert_eq!(fetched.content, content);
        assert_eq!(fetched.code, code);
        Ok(())
    }

    #[sqlx::test]
    async fn get_course_by_id_returns_none_for_unknown_id(pool: PgPool) -> sqlx::Result<()> {
        let result = get_course_by_id(&pool, Uuid::new_v4()).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[sqlx::test]
    async fn get_course_by_code_returns_none_for_unknown_code(pool: PgPool) -> sqlx::Result<()> {
        let result = get_course_by_code(&pool, "Unknown").await?;
        assert!(result.is_none());
        Ok(())
    }

    #[sqlx::test]
    async fn delete_course_removes_course(pool: PgPool) -> sqlx::Result<()> {
        let name = "ToDelete".to_string();
        let content = Some("Course content".to_string());
        let code = "C103".to_string();
        create_course(&pool, name.clone(), content.clone(), code.clone()).await?;

        let course = get_course_by_code(&pool, &code)
            .await?
            .expect("course should exist");

        delete_course(&pool, course.id).await?;

        let result = get_course_by_id(&pool, course.id).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[sqlx::test]
    async fn update_course_changes_fields(pool: PgPool) -> sqlx::Result<()> {
        create_course(
            &pool,
            "Old".to_string(),
            Some("Old content".to_string()),
            "C200".to_string(),
        )
        .await?;

        let course = get_course_by_code(&pool, "C200")
            .await?
            .expect("course should exist");

        update_course(
            &pool,
            course.id,
            "New".to_string(),
            Some("New content".to_string()),
            "C201".to_string(),
        )
        .await?;

        let updated = get_course_by_id(&pool, course.id)
            .await?
            .expect("course should exist");
        assert_eq!(updated.name, "New");
        assert_eq!(updated.content, Some("New content".to_string()));
        assert_eq!(updated.code, "C201");
        Ok(())
    }
}
