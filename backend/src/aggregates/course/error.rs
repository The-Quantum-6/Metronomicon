use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub struct CourseError(String);

impl From<&str> for CourseError {
    fn from(value: &str) -> Self {
        CourseError(value.to_string())
    }
}

impl Display for CourseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}