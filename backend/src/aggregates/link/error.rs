use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub struct LinkError(String);

impl Display for LinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for LinkError {
    fn from(value: &str) -> Self {
        LinkError(value.to_string())
    }
}