use super::super::strategies::RegistrationStrategy;
use super::super::strategies::EmailRegistrationStrategy;

use super::super::dto::EmailRegistrationDTO;
use super::super::errors::RegistrationValidationError;

pub struct RegistrationService<T, U> {
    strategy: Box<dyn RegistrationStrategy<T, U>>,
}

impl<T, U> RegistrationService<T, U> {
    pub fn register(&self, dto: &T) -> U {
        self.strategy.register(dto)
    }
}

pub struct RegistrationServiceFactory;

impl RegistrationServiceFactory {
    pub fn create_for_email_strategy() -> RegistrationService<EmailRegistrationDTO, Result<(), RegistrationValidationError>> {
        let strategy = Box::new(EmailRegistrationStrategy);

        RegistrationService {
            strategy
        }
    }
}