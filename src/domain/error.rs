use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ValidationError {
    message: String,
}

impl ValidationError {
    pub fn new(massage: &str) -> Self {
        ValidationError {
            message: massage.to_string(),
        }
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

impl Error for ValidationError {}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug)]
pub struct EntityConflict {}

impl Error for EntityConflict {}

impl Display for EntityConflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity already exist")
    }
}

#[derive(Debug)]
pub struct EntityNotExist {}

impl Error for EntityNotExist {}

impl Display for EntityNotExist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity do not exist")
    }
}
