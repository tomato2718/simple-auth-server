use crate::application::service::password::PasswordHasher;
use crate::domain::{entity::User, error, repository::UserRepository, value_object::EmailAddress};

pub struct SignUpUseCase<'a> {
    password_hasher: &'a dyn PasswordHasher,
    user_repository: &'a mut dyn UserRepository,
    get_timestamp: fn() -> u64,
}

impl<'a> SignUpUseCase<'a> {
    pub fn new(
        password_hasher: &'a dyn PasswordHasher,
        user_repository: &'a mut dyn UserRepository,
        get_timestamp: fn() -> u64,
    ) -> Self {
        SignUpUseCase {
            password_hasher,
            user_repository,
            get_timestamp,
        }
    }

    pub fn execute(&mut self, user_data: CreateUserDTO) -> Result<(), error::EntityConflict> {
        let now = (self.get_timestamp)();
        let user = User {
            email: user_data.email_address,
            username: user_data.username,
            password: self.password_hasher.hash(&user_data.password),
            create_at: now,
            update_at: now,
        };
        self.user_repository.create(user)
    }
}

pub struct CreateUserDTO {
    pub email_address: EmailAddress,
    pub username: String,
    pub password: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    struct FakeUserRepository {
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

    struct FakePasswordHasher {}

    impl PasswordHasher for FakePasswordHasher {
        fn hash(&self, raw: &str) -> String {
            raw.to_string()
        }
    }

    fn fake_get_timestamp() -> u64 {
        1747636936
    }

    #[test]
    fn execute_given_user_information_should_persist_to_repository() {
        let mut mock_user_repository = FakeUserRepository::new();
        let mut sign_up = SignUpUseCase::new(
            &FakePasswordHasher{},
            &mut mock_user_repository,
            fake_get_timestamp,
        );
        let user = CreateUserDTO {
            email_address: EmailAddress::new("example@example.com").unwrap(),
            username: "test".to_string(),
            password: "password".to_string(),
        };

        sign_up.execute(user).unwrap();

        assert!(
            mock_user_repository
                .data
                .contains_key("example@example.com")
        )
    }
}
