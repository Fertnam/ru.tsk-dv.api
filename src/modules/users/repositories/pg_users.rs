use crate::establish_connection;
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
}