use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct EmailRegistrationDTO {
    pub name: String,

    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    
    pub password: String
}