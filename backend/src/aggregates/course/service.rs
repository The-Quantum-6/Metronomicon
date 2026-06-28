use sqlx::{Pool, Postgres};

pub struct CourseServices(pub Pool<Postgres>);

impl CourseServices {
    pub async fn course_exists(&self, id: &str) -> Result<bool, sqlx::Error> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(
                SELECT 1
                FROM course_list_view
                WHERE aggregate_id = $1
            )",
            id
        )
        .fetch_one(&self.0)
        .await?;

        Ok(exists.unwrap_or(false))
    }
}
