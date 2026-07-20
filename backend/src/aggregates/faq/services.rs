use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct FaqExistanceService(pub Pool<Postgres>);

impl FaqExistanceService {
    pub async fn faq_exists(&self, course_id: &str, faq_id: &Uuid) -> Result<bool, sqlx::Error> {
        let faq_id_str = faq_id.to_string();
        let exists: Option<bool> = sqlx::query_scalar(
            "SELECT EXISTS(
                SELECT 1
                FROM course_detail_view,
                     jsonb_array_elements(payload->'faqs') AS faq
                WHERE view_id = $1
                  AND payload->>'status' = 'Active'
                  AND faq->>'faq_id' = $2
            )",
        )
        .bind(course_id)
        .bind(&faq_id_str)
        .fetch_one(&self.0)
        .await?;

        Ok(exists.unwrap_or(false))
    }
}
