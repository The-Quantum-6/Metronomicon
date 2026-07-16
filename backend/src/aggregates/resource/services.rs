use crate::aggregates::resource::error::ResourceError;
use crate::storage::Storage;

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
