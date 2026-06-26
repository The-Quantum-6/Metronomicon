use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub struct ResourceError(String);

impl Display for ResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ResourceError {
    fn from(value: &str) -> Self {
        ResourceError(value.to_string())
    }
}
