use validator::Validate;

use super::RegistrationStrategy;
use super::super::dto::EmailRegistrationDTO;
use super::super::errors::RegistrationValidationError;

pub struct EmailRegistrationStrategy;

impl RegistrationStrategy<EmailRegistrationDTO, Result<(), RegistrationValidationError>> for EmailRegistrationStrategy {
    fn register(&self, dto: &EmailRegistrationDTO) -> Result<(), RegistrationValidationError> {
        match dto.validate() {
            Ok(_) => {
                println!("Registering user via Email: {}", dto.email);
                Ok(())
            },
            Err(errors) => {
                Err(RegistrationValidationError::new(String::from("Validation Error"), errors))
            }
        }
    }
}
