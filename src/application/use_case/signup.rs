use crate::application::service::auth::PasswordHasher;
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
    use crate::test_support::{
        application::service::FakePasswordHasher, domain::repository::FakeUserRepository,
    };

    fn fake_get_timestamp() -> u64 {
        1747636936
    }

    #[test]
    fn execute_given_user_information_should_persist_to_repository() {
        let mut mock_user_repository = FakeUserRepository::new();
        let mut sign_up = SignUpUseCase::new(
            &FakePasswordHasher {},
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

    #[test]
    fn execute_given_conflict_email_should_return_entity_conflict() {
        let mut mock_user_repository = FakeUserRepository::new();
        mock_user_repository.data.insert(
            "example@example.com".to_string(),
            User {
                email: EmailAddress::new("example@example.com").unwrap(),
                username: "foo".to_string(),
                password: "bar".to_string(),
                create_at: 1747636936,
                update_at: 1747636936,
            },
        );
        let user = CreateUserDTO {
            email_address: EmailAddress::new("example@example.com").unwrap(),
            username: "test".to_string(),
            password: "password".to_string(),
        };
        let mut sign_up = SignUpUseCase::new(
            &FakePasswordHasher {},
            &mut mock_user_repository,
            fake_get_timestamp,
        );

        let result = sign_up.execute(user);

        assert!(matches!(result, Err(error::EntityConflict {})));
    }
}
