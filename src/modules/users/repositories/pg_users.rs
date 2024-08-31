use crate::establish_connection;
use super::super::dto::UserCreationDTO;
use diesel::prelude::*;
use super::super::models::User;
use super::UsersRepository;

pub struct PgUsersRepository;

impl UsersRepository for PgUsersRepository {
    fn find_all(&self) -> Vec<User> {
        use crate::schema::users::dsl::*;

        let connection = &mut establish_connection();
        let results = users
            .select(User::as_select())
            .load(connection)
            .expect("Error loading users");

        results
    }

    fn save(&self, dto: &UserCreationDTO) -> User {
        use crate::schema::users;

        let connection = &mut establish_connection();

        diesel::insert_into(users::table)
            .values(dto)
            .returning(User::as_returning())
            .get_result(connection)
            .expect("Error saving new user")
    }
}