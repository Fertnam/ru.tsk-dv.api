use super::super::dto::UserCreationDTO;
use super::super::models::User;

pub trait UsersRepository {
    fn find_all(&self) -> Vec<User>;
    fn save(&self, dto: &UserCreationDTO) -> User;
}