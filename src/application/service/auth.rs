pub trait PasswordHasher {
    fn hash(&self, raw: &str) -> String;
}

pub trait PasswordValidator {
    fn verify(&self, raw: &str, hashed: &str) -> bool;
}

pub trait TokenIssuer {
    fn issue(&self, username: &str) -> String;
}
