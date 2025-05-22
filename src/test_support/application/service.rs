use crate::application::service::auth::{PasswordHasher, TokenIssuer};

pub struct FakePasswordHasher {}

impl PasswordHasher for FakePasswordHasher {
    fn hash(&self, raw: &str) -> String {
        raw.to_string()
    }
}

pub struct FakeTokenIssuer {
    to_return: String
}

impl FakeTokenIssuer {
    fn new(to_return: &str) -> Self {
        FakeTokenIssuer { to_return: to_return.to_string() }
    }
}

impl TokenIssuer for FakeTokenIssuer {
    fn issue(&self, username: &str) -> String {
        self.to_return.clone()
    }
}