use crate::aggregates::link::error::LinkError;

pub struct LinkServices;

impl LinkServices {
    /// TODO: Check whether a link is valid
    async fn check_valid(&self, url: &str, client: reqwest::Client) -> Result<(), LinkError> {
        Ok(())
    }
}