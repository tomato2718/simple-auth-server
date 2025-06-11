use std::env;

pub struct EnvVar {
    pub app_name: String,
    pub access_token_secret: Vec<u8>,
    pub access_token_valid_seconds: u64,
    pub refresh_token_secret: Vec<u8>,
    pub refresh_token_valid_seconds: u64,
}

pub fn get_envvar() -> EnvVar {
    EnvVar {
        app_name: env::var("APP_NAME").unwrap(),
        access_token_secret: env::var("ACCESS_TOKEN_SECRET").unwrap().as_bytes().to_vec(),
        access_token_valid_seconds: env::var("ACCESS_TOKEN_VALID_SECONDS")
            .unwrap()
            .parse()
            .unwrap(),
        refresh_token_secret: env::var("REFRESH_TOKEN_SECRET")
            .unwrap()
            .as_bytes()
            .to_vec(),
        refresh_token_valid_seconds: env::var("REFRESH_TOKEN_VALID_SECONDS")
            .unwrap()
            .parse()
            .unwrap(),
    }
}
