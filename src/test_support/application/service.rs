use crate::application::service::auth::{PasswordHasher, TokenIssuer};

pub struct FakePasswordHasher {
    to_return: String,
}

impl FakePasswordHasher {
    pub fn new(to_return: &str) -> Self {
        FakePasswordHasher {
            to_return: to_return.to_string(),
        }
    }
}

impl PasswordHasher for FakePasswordHasher {
    fn hash(&self, raw: &str) -> String {
        self.to_return.clone()
    }
}

pub struct FakeTokenIssuer {
    to_return: String,
}

impl FakeTokenIssuer {
    pub fn new(to_return: &str) -> Self {
        FakeTokenIssuer {
            to_return: to_return.to_string(),
        }
    }
}

impl TokenIssuer for FakeTokenIssuer {
    fn issue(&self, username: &str) -> String {
        self.to_return.clone()
    }
}
