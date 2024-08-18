use validator::Validate;

use super::RegistrationStrategy;
use super::super::dto::EmailRegistrationDTO;
use super::super::errors::RegistrationError;

pub struct EmailRegistrationStrategy;

impl RegistrationStrategy<EmailRegistrationDTO> for EmailRegistrationStrategy {
    fn register(&self, dto: &EmailRegistrationDTO) -> Result<(), RegistrationError> {
        dto.validate().map_err(|errors| RegistrationError::FailedValidation(errors))?;

        println!("Registering user via Email: {}", dto.email);
        Ok(())
    } 
}
