use super::super::strategies::RegistrationStrategy;
use super::super::strategies::EmailRegistrationStrategy;

use super::super::dto::EmailRegistrationDTO;
use super::super::errors::RegistrationError;

pub struct RegistrationService<T> {
    strategy: Box<dyn RegistrationStrategy<T>>,
}

impl<T> RegistrationService<T> {
    pub fn register(&self, dto: &T) -> Result<(), RegistrationError> {
        self.strategy.register(dto)
    }
}

pub struct RegistrationServiceFactory;

impl RegistrationServiceFactory {
    pub fn create_for_email_strategy() -> RegistrationService<EmailRegistrationDTO> {
        let strategy = Box::new(EmailRegistrationStrategy);

        RegistrationService {
            strategy
        }
    }
}