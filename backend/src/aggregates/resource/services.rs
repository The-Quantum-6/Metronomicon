use crate::aggregates::resource::error::ResourceError;
use crate::storage::Storage;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

/// Wraps object storage (Garage) so the resource aggregate can verify that
/// an uploaded file actually exists before recording an event referencing it.
pub struct ResourceServices(pub Storage);

impl ResourceServices {
    /// Ok(()) if an object exists under `key` in Garage.
    ///
    /// Distinguishes "file not there" (a normal validation failure the user
    /// caused by sending a bad key) from "storage unreachable" (an
    /// operational error) — both are errors to the aggregate, but the
    /// messages differ so logs make sense.
    pub async fn check_exists(&self, key: &str) -> Result<(), ResourceError> {
        match self.0.exists(key).await {
            Ok(true) => Ok(()),
            Ok(false) => Err("resource file not found".into()),
            Err(e) => Err(format!("storage unavailable: {e}").as_str().into()),
        }
    }
}

pub struct ResourceExistanceService(pub Pool<Postgres>);

impl ResourceExistanceService {
    pub async fn resource_exists(
        &self,
        course_id: &str,
        resource_id: &Uuid,
    ) -> Result<bool, sqlx::Error> {
        let resource_id_str = resource_id.to_string();
        let exists: Option<bool> = sqlx::query_scalar(
            "SELECT EXISTS(
                SELECT 1
                FROM course_detail_view,
                     jsonb_array_elements(payload->'resources') AS resource
                WHERE view_id = $1
                  AND payload->>'status' = 'Active'
                  AND resource->>'resource_id' = $2
            )",
        )
        .bind(course_id)
        .bind(&resource_id_str)
        .fetch_one(&self.0)
        .await?;

        Ok(exists.unwrap_or(false))
    }
}
