use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct ProjectIdeaExistanceService(pub Pool<Postgres>);

impl ProjectIdeaExistanceService {
    pub async fn project_idea_exists(&self, course_id: &str, project_idea_id: &Uuid) -> Result<bool, sqlx::Error> {
        let project_idea_id_str = project_idea_id.to_string();
        let exists: Option<bool> = sqlx::query_scalar(
            "SELECT EXISTS(
                SELECT 1
                FROM course_detail_view,
                     jsonb_array_elements(payload->'project_ideas') AS project_idea
                WHERE view_id = $1
                  AND payload->>'status' = 'Active'
                  AND project_idea->>'idea_id' = $2
            )",
        )
        .bind(course_id)
        .bind(&project_idea_id_str)
        .fetch_one(&self.0)
        .await?;

        Ok(exists.unwrap_or(false))
    }
}
