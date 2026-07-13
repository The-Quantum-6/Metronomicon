use crate::aggregates::link::error::LinkError;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct LinkValidityService(pub reqwest::Client);

impl LinkValidityService {
    /// Checks whether a URL is valid by sending a HEAD request and verifying the response.
    pub async fn check_valid(&self, url: &str) -> Result<(), LinkError> {
        let response = self.0.head(url).send().await.map_err(|e| e.to_string())?;

        if response.status().is_success() || response.status().is_redirection() {
            Ok(())
        } else {
            Err(response.status().as_str().into())
        }
    }
}

pub struct LinkExistanceService(pub Pool<Postgres>);

impl LinkExistanceService {
    pub async fn link_exists(&self, course_id: &str, link_id: &Uuid) -> Result<bool, sqlx::Error> {
        let link_id_str = link_id.to_string();
        let exists: Option<bool> = sqlx::query_scalar!(
            "SELECT EXISTS(
                SELECT 1
                FROM course_detail_view,
                     jsonb_array_elements(payload->'links') AS link
                WHERE view_id = $1
                  AND payload->>'status' = 'Active'
                  AND link->>'link_id' = $2
            )",
            course_id,
            &link_id_str
        )
        .fetch_one(&self.0)
        .await?;

        Ok(exists.unwrap_or(false))
    }
}

pub struct LinkServices(pub LinkValidityService, pub LinkExistanceService);
