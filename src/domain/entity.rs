use super::value_object::EmailAddress;

pub struct User {
    email: EmailAddress,
    username: String,
    password: Vec<u8>,
}
