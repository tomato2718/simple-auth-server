pub trait PasswordHasher {
    fn hash(raw: &str) -> String;
}

pub trait PasswordValidator {
    fn verify(raw: &str, hashed: &str) -> bool;
}
