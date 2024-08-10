use super::super::strategies::RegistrationStrategy;
use super::super::strategies::EmailRegistrationStrategy;

use super::super::dto::EmailRegistrationDTO;

pub struct RegistrationService<T> {
    strategy: Box<dyn RegistrationStrategy<T>>,
}

impl<T> RegistrationService<T> {
    pub fn register(&self, dto: &T) {
        self.strategy.register(dto);
    }
}

pub struct RegistrationServiceFactory;

impl RegistrationServiceFactory {
    pub fn create_for_email_strategy() -> RegistrationService<EmailRegistrationDTO>  {
        let strategy = Box::new(EmailRegistrationStrategy {});

        RegistrationService {
            strategy
        }
    }
}