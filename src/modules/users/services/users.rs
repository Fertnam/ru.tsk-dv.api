use super::super::repositories::{UsersRepository, PgUsersRepository};
use super::super::models::User;

pub struct UsersService {
    users_repository: Box<dyn UsersRepository>,
}

impl UsersService {
    pub fn find_all(&self) -> Vec<User> {
        self.users_repository.find_all()
    }
}

pub struct UsersServiceFactory;

impl UsersServiceFactory {
    pub fn create_for_pg_users_repository() -> UsersService {
        let users_repository = Box::new(PgUsersRepository);

        UsersService {
            users_repository
        }
    }
}