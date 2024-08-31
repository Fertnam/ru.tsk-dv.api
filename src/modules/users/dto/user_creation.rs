use crate::schema::users;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct UserCreationDTO {
    pub name: String,
    pub email: String,
    pub password: String
}