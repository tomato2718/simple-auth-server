use crate::application::service::auth::PasswordHasher;

pub struct FakePasswordHasher {}

impl PasswordHasher for FakePasswordHasher {
    fn hash(&self, raw: &str) -> String {
        raw.to_string()
    }
}
