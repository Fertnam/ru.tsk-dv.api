use serde::Deserialize;

#[derive(Deserialize)]
pub struct EmailRegistrationDTO {
    pub name: String,
    pub email: String,
    pub password: String
}