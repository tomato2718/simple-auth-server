use crate::domain::{
    entity::User,
    error::{EntityConflict, EntityNotExist},
    repository::UserRepository,
    value_object::EmailAddress,
};
use std::collections::HashMap;

pub struct InMemoryUserRepository {
    data: HashMap<String, User>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        InMemoryUserRepository {
            data: HashMap::new(),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    fn create(&mut self, user: User) -> Result<(), EntityConflict> {
        if self.data.contains_key(user.email.as_str()) {
            return Err(EntityConflict {});
        }
        self.data.insert(user.email.as_str().to_string(), user);

        Ok(())
    }

    fn get(&self, email: EmailAddress) -> Result<User, EntityNotExist> {
        match self.data.get(email.as_str()) {
            Some(user) => Ok(User {
                email: EmailAddress::new(user.email.as_str()).unwrap(),
                username: user.username.clone(),
                password: user.password.clone(),
                create_at: user.create_at,
                update_at: user.update_at,
            }),
            None => Err(EntityNotExist {}),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_user() -> User {
        User {
            email: EmailAddress::new("example@example.com").unwrap(),
            username: "foo".to_string(),
            password: "bar".to_string(),
            create_at: 1747636936,
            update_at: 1747636936,
        }
    }

    #[test]
    fn create_given_user_should_persist_to_data() {
        let mut repo = InMemoryUserRepository::new();
        let user = create_user();

        let result = repo.create(user);

        assert!(result.is_ok());
    }

    #[test]
    fn create_given_conflict_email_should_return_entity_conflict() {
        let mut repo = InMemoryUserRepository::new();
        repo.create(create_user()).expect("should be ok");
        let user = create_user();

        let result = repo.create(user);

        assert!(result.is_err_and(|err| matches!(err, EntityConflict {})));
    }

    #[test]
    fn get_given_email_should_return_user() {
        let mut repo = InMemoryUserRepository::new();
        repo.create(create_user()).expect("should be ok");

        let user = repo.get(EmailAddress::new("example@example.com").unwrap());

        assert!(user.is_ok());
    }

    #[test]
    fn get_given_not_exist_email_should_return_entity_not_exist() {
        let repo = InMemoryUserRepository::new();

        let user = repo.get(EmailAddress::new("example@example.com").unwrap());

        assert!(user.is_err_and(|err| matches!(err, EntityNotExist {})));
    }
}
