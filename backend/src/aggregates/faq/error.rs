use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub struct FaqError(String);

impl Display for FaqError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for FaqError {
    fn from(value: &str) -> Self {
        FaqError(value.to_string())
    }
}