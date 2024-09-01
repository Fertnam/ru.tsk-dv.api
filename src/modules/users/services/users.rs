use super::super::dto::UserCreationDTO;
use super::super::repositories::UsersRepository;
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
    pub fn create<T: UsersRepository + 'static>(users_repository_creator: fn() -> T) -> UsersService {
        let users_repository = Box::new(users_repository_creator());

        UsersService {
            users_repository
        }
    }
}