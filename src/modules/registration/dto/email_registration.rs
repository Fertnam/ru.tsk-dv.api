use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct EmailRegistrationDTO {
    pub name: String,

    #[validate(email(message = "Невалидный email"), length(max = 350, message = "Максимальная длина 350 символов"))]
    pub email: String,
    
    #[validate(length(max = 128, message = "Максимальная длина 128 символов"))]
    pub password: String
}