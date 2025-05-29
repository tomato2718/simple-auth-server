use crate::application::service::auth::{PasswordHasher, PasswordValidator};
use bcrypt::{hash, verify};

struct BcryptHasher {
    round: u32,
}

impl BcryptHasher {
    pub fn new(round: u32) -> BcryptHasher {
        BcryptHasher { round }
    }
}

impl PasswordHasher for BcryptHasher {
    fn hash(&self, raw: &str) -> String {
        hash(raw, self.round).unwrap()
    }
}

struct BcryptValidator {}

impl PasswordValidator for BcryptValidator {
    fn verify(&self, raw: &str, hashed: &str) -> bool {
        verify(raw, hashed).is_ok()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bcrypt_hasher_given_raw_password_should_return_hashed_password() {
        let password = "password";
        let hasher = BcryptHasher::new(4);

        let hashed_password = hasher.hash(&password);

        assert!(verify(password, hashed_password.as_str()).is_ok());
    }

    #[test]
    fn bcrypt_validator_given_raw_and_hashed_password_should_return_is_valid() {
        let passwords = vec!["correct_password", "wrong_password"];
        let validator = BcryptValidator {};

        for p in passwords {
            assert!(validator.verify(
                p,
                "$2a$04$FlenKTKcUW/BI0HBwCPTReMLMh0uo8zuKfja7N.Uo3IHjM3Kp0SIK"
            ));
        }
    }
}
