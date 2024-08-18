use validator::ValidationErrors;
use std::fmt;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum RegistrationError {
    FailedValidation(ValidationErrors)
}

impl fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistrationError::FailedValidation(errors) => write!(f, "RegistrationError::FailedValidation: {:?}", errors),
        }
    }
}