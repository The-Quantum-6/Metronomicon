use crate::aggregates::resource::error::ResourceError;

pub struct ResourceServices(pub reqwest::Client);

impl ResourceServices {
    /// TODO: check whether an object exists in Garage/S3
    pub async fn check_exists(&self, _key: &str) -> Result<(), ResourceError> {
        Ok(())
    }
}
