use super::super::errors::RegistrationError;

pub trait RegistrationStrategy<T> {
    fn register(&self, dto: &T) -> Result<(), RegistrationError>;
}