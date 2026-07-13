use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub struct ContributionError(String);

impl Display for ContributionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ContributionError {
    fn from(value: &str) -> Self {
        ContributionError(value.to_string())
    }
}

impl From<String> for ContributionError {
    fn from(value: String) -> Self {
        ContributionError(value)
    }
}
