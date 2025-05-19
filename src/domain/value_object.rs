use super::error::ValidationError;

pub struct EmailAddress {
    address: String,
}

impl EmailAddress {
    pub fn new(address: &str) -> Result<Self, ValidationError> {
        if !EmailAddress::validate(address) {
            return Err(ValidationError::new("Invalid email address"));
        }
        Ok(EmailAddress {
            address: address.to_string(),
        })
    }

    fn validate(address: &str) -> bool {
        let address_vec: Vec<&str> = address.split("@").collect();
        if address_vec.len() != 2 {
            return false;
        }

        let local_part = address_vec[0];
        let domain = address_vec[1];

        if local_part.is_empty()
            || local_part.len() > 64
            || local_part.starts_with('.')
            || local_part.ends_with('.')
            || local_part.contains("..")
            || !local_part
                .chars()
                .all(|c| c.is_alphanumeric() || "!#$%&'*+-/=?^_`{|}~.".contains(c))
        {
            return false;
        }

        if domain.is_empty()
            || domain.len() > 255
            || domain.starts_with('.')
            || domain.ends_with('.')
            || !domain.contains('.')
            || !domain
                .chars()
                .all(|c| c.is_alphanumeric() || c == '.' || c == '-')
            || domain
                .split('.')
                .any(|d| d.starts_with('-') || d.ends_with('-'))
        {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod test_email_address {
    use super::EmailAddress;

    #[test]
    fn new_given_valid_email_should_return_instance() {
        let test_cases = vec![
            "user@example.com",
            "user.name@example.com",
            "user+tag@example.co.uk",
            "user_name@example-domain.com",
            "user123@server.net",
        ];

        for test_case in test_cases {
            assert!(EmailAddress::new(test_case).is_ok());
        }
    }

    #[test]
    fn new_given_invalid_email_format_should_return_error() {
        let test_cases = vec![
            "user@",
            "@example.com",
            "user@.com",
            "user@domain",
            "user..name@example.com",
            ".user@example.com",
            "user@-example.com",
            "user@example-.com",
            "user name@example.com",
        ];

        for test_case in test_cases {
            assert!(EmailAddress::new(test_case).is_err());
        }
    }
}
