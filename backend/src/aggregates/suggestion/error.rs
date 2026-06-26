use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub struct SuggestionError(String);

impl Display for SuggestionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for SuggestionError {
    fn from(value: &str) -> Self {
        SuggestionError(value.to_string())
    }
}
