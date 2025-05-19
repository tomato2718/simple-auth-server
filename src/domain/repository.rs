use super::{entity::User, error, value_object::EmailAddress};

pub trait UserRepository {
    fn create(&self, user: User) -> Result<(), error::EntityConflict>;

    fn get(&self, email: EmailAddress) -> Result<User, error::EntityNotExist>;
}
