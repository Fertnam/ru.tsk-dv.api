use super::RegistrationStrategy;
use super::super::dto::EmailRegistrationDTO;

pub struct EmailRegistrationStrategy;

impl RegistrationStrategy<EmailRegistrationDTO> for EmailRegistrationStrategy {
    fn register(&self, dto: &EmailRegistrationDTO) {
        println!("Registering user via Email: {}", dto.email);
        // Логика регистрации
    }
}
