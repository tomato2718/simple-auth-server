use super::value_object::EmailAddress;

pub struct User {
    pub email: EmailAddress,
    pub username: String,
    pub password: String,
    pub create_at: u64,
    pub update_at: u64,
}
