use crate::aggregates::link::error::LinkError;

pub struct LinkServices(pub reqwest::Client);

impl LinkServices {
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
