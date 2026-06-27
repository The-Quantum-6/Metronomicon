use uuid::Uuid;

use crate::aggregates::resource::error::ResourceError;

pub struct ResourceServices;

impl ResourceServices {
    /// TODO: check whether an object exists in S3
    async fn check_exists(
        &self,
        _key: Uuid,
        _client: reqwest::Client,
    ) -> Result<(), ResourceError> {
        Ok(())
    }
}
