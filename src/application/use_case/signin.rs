use crate::application::service::{auth::PasswordValidator, auth::TokenIssuer};
use crate::domain::{entity::User, error, repository::UserRepository, value_object::EmailAddress};

pub struct SignInUseCase<'a> {
    password_validator: &'a dyn PasswordValidator,
    access_token_issuer: &'a dyn TokenIssuer,
    refresh_token_issuer: &'a dyn TokenIssuer,
    user_repository: &'a dyn UserRepository,
}

impl<'a> SignInUseCase<'a> {
    pub fn new(
        password_validator: &'a dyn PasswordValidator,
        access_token_issuer: &'a dyn TokenIssuer,
        refresh_token_issuer: &'a dyn TokenIssuer,
        user_repository: &'a dyn UserRepository,
    ) -> Self {
        SignInUseCase {
            password_validator,
            access_token_issuer,
            refresh_token_issuer,
            user_repository,
        }
    }

    pub fn execute(self, email: EmailAddress, password: &str) -> Result<SignInResult, FailReason> {
        let user = match self.user_repository.get(email) {
            Ok(u) => u,
            Err(_) => return Err(FailReason::UserNotExist),
        };
        if !self.password_validator.verify(password, &user.password) {
            return Err(FailReason::InvalidPassowrd);
        }

        Ok(SignInResult {
            access_token: self.access_token_issuer.issue(&user.username),
            refresh_token: self.refresh_token_issuer.issue(&user.username),
            username: user.username,
            email: user.email.as_str().to_string(),
        })
    }
}

pub struct SignInResult {
    access_token: String,
    refresh_token: String,
    username: String,
    email: String,
}

pub enum FailReason {
    UserNotExist,
    InvalidPassowrd,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_support::{
        application::service::{FakePasswordValidator, FakeTokenIssuer},
        domain::repository::FakeUserRepository,
    };

    fn setup_repository() -> FakeUserRepository {
        let mut repo = FakeUserRepository::new();
        repo.data.insert(
            "example@example.com".to_string(),
            User {
                email: EmailAddress::new("example@example.com").unwrap(),
                username: "foo".to_string(),
                password: "bar".to_string(),
                create_at: 1747636936,
                update_at: 1747636936,
            },
        );
        repo
    }

    #[test]
    fn execute_given_valid_user_data_should_return_signin_result() {
        let stub_access_token_issuer = FakeTokenIssuer::new("access_token");
        let stub_refresh_token_issuer = FakeTokenIssuer::new("refresh_token");
        let stub_password_validator = FakePasswordValidator::new(true);
        let mut mock_user_repository = setup_repository();
        let sign_in = SignInUseCase::new(
            &stub_password_validator,
            &stub_access_token_issuer,
            &stub_refresh_token_issuer,
            &mut mock_user_repository,
        );

        let result = sign_in.execute(
            EmailAddress::new("example@example.com").unwrap(),
            "password",
        );

        assert!(result.is_ok_and(|r| r.email == "example@example.com"
            && r.username == "foo"
            && r.access_token == "access_token"
            && r.refresh_token == "refresh_token"));
    }

    #[test]
    fn execute_given_not_exist_user_should_return_entity_not_exist() {
        let stub_access_token_issuer = FakeTokenIssuer::new("access_token");
        let stub_refresh_token_issuer = FakeTokenIssuer::new("refresh_token");
        let stub_password_validator = FakePasswordValidator::new(true);
        let mut mock_user_repository = setup_repository();
        let sign_in = SignInUseCase::new(
            &stub_password_validator,
            &stub_access_token_issuer,
            &stub_refresh_token_issuer,
            &mut mock_user_repository,
        );

        let result = sign_in.execute(
            EmailAddress::new("not_exist@example.com").unwrap(),
            "password",
        );

        assert!(result.is_err_and(|err| matches!(err, FailReason::UserNotExist)));
    }

    #[test]
    fn execute_given_invalid_password_should_return_entity_not_exist() {
        let stub_access_token_issuer = FakeTokenIssuer::new("access_token");
        let stub_refresh_token_issuer = FakeTokenIssuer::new("refresh_token");
        let mock_password_validator = FakePasswordValidator::new(false);
        let mut stub_user_repository = setup_repository();
        let sign_in = SignInUseCase::new(
            &mock_password_validator,
            &stub_access_token_issuer,
            &stub_refresh_token_issuer,
            &mut stub_user_repository,
        );

        let result = sign_in.execute(
            EmailAddress::new("example@example.com").unwrap(),
            "password",
        );

        assert!(result.is_err_and(|err| matches!(err, FailReason::InvalidPassowrd)));
    }
}
