use super::super::models::User;

pub trait UsersRepository {
    fn find_all(&self) -> Vec<User>;
}