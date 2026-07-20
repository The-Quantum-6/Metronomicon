use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReportError {
    #[error("{0}")]
    UserError(String),
}

impl From<&str> for ReportError {
    fn from(msg: &str) -> Self {
        ReportError::UserError(msg.to_string())
    }
}
