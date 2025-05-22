use crate::application::service::password::PasswordHasher;

pub struct FakePasswordHasher {}

impl PasswordHasher for FakePasswordHasher {
    fn hash(&self, raw: &str) -> String {
        raw.to_string()
    }
}
