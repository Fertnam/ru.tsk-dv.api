use validator::ValidationErrors;
use std::fmt;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RegistrationValidationError {
    context: String,
    errors: ValidationErrors,
}

impl RegistrationValidationError {
    pub fn new(context: String, errors: ValidationErrors) -> RegistrationValidationError {
        RegistrationValidationError {
            context,
            errors
        }
    }
}

impl fmt::Display for RegistrationValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.context, self.errors)
    }
}