use super::super::strategies::RegistrationStrategy;

pub struct RegistrationService<T> {
    strategy: Box<dyn RegistrationStrategy<T>>,
}

impl<T> RegistrationService<T> {
    pub fn new(strategy: Box<dyn RegistrationStrategy<T>>) -> Self {
        Self { 
            strategy 
        }
    }

    pub fn register(&self, dto: &T) {
        self.strategy.register(dto);
    }
}