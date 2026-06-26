use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub struct ProjectIdeaError(String);

impl Display for ProjectIdeaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ProjectIdeaError {
    fn from(value: &str) -> Self {
        ProjectIdeaError(value.to_string())
    }
}
