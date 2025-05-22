use crate::domain::{entity::User, error, repository::UserRepository, value_object::EmailAddress};
use std::collections::HashMap;

pub struct FakeUserRepository {
    pub data: HashMap<String, User>,
}

impl FakeUserRepository {
    pub fn new() -> Self {
        FakeUserRepository {
            data: HashMap::new(),
        }
    }
}

impl UserRepository for FakeUserRepository {
    fn create(&mut self, user: User) -> Result<(), error::EntityConflict> {
        if self.data.contains_key(user.email.as_str()) {
            return Err(error::EntityConflict {});
        }
        self.data.insert(user.email.as_str().to_string(), user);

        Ok(())
    }

    fn get(&self, email: EmailAddress) -> Result<User, error::EntityNotExist> {
        match self.data.get(email.as_str()) {
            Some(user) => Ok(User {
                email: EmailAddress::new(user.email.as_str()).unwrap(),
                username: user.username.clone(),
                password: user.password.clone(),
                create_at: user.create_at,
                update_at: user.update_at,
            }),
            None => Err(error::EntityNotExist {}),
        }
    }
}
