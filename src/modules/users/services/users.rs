use super::super::dto::UserCreationDTO;
use super::super::repositories::{UsersRepository, PgUsersRepository};
use super::super::models::User;

pub struct UsersService {
    users_repository: Box<dyn UsersRepository>,
}

impl UsersService {
    pub fn find_all(&self) -> Vec<User> {
        self.users_repository.find_all()
    }

    pub fn create(&self, dto: &UserCreationDTO) -> User {
        self.users_repository.save(dto)
    }
}

pub struct UsersServiceFactory;

impl UsersServiceFactory {
    pub fn create_for_pg() -> UsersService {
        let users_repository = Box::new(PgUsersRepository);

        UsersService {
            users_repository
        }
    }
}