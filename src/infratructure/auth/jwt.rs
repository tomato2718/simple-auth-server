use crate::application::service::auth::TokenIssuer;
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token, header::HeaderType};
use sha2::Sha256;

pub struct JWTIssuer<'a> {
    secret: &'a [u8],
    infra_claims: InfraClaims,
}

impl<'a> JWTIssuer<'a> {
    pub fn new(secret: &'a [u8], infra_claims: InfraClaims) -> Self {
        JWTIssuer {
            secret,
            infra_claims,
        }
    }
}

impl<'a> TokenIssuer for JWTIssuer<'a> {
    fn issue(&self, username: &str) -> String {
        let mac: Hmac<Sha256> = Hmac::new_from_slice(&self.secret).unwrap();
        let header = Header {
            algorithm: AlgorithmType::Hs256,
            type_: Some(HeaderType::JsonWebToken),
            ..Default::default()
        };
        let payload = CompleteClaims {
            sub: username,
            iss: &self.infra_claims.iss,
            aud: &self.infra_claims.aud,
            iat: &self.infra_claims.iat,
            exp: &self.infra_claims.exp,
        };
        Token::new(header, payload)
            .sign_with_key(&mac)
            .unwrap()
            .as_str()
            .to_string()
    }
}

pub struct InfraClaims {
    pub iss: String,
    pub aud: String,
    pub iat: u32,
    pub exp: u32,
}

#[derive(serde::Serialize)]
struct CompleteClaims<'a> {
    pub sub: &'a str,
    pub iss: &'a str,
    pub aud: &'a str,
    pub iat: &'a u32,
    pub exp: &'a u32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn issue_given_username_should_issue_jwt() {
        let secret = b"secret";
        let infra_claims = InfraClaims {
            iss: "example".to_string(),
            aud: "example".to_string(),
            iat: 1516239022,
            exp: 1516325422,
        };
        let issuer = JWTIssuer::new(secret, infra_claims);

        let token = issuer.issue("username");

        assert_eq!(
            token,
            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VybmFtZSIsImlzcyI6ImV4YW1wbGUiLCJhdWQiOiJleGFtcGxlIiwiaWF0IjoxNTE2MjM5MDIyLCJleHAiOjE1MTYzMjU0MjJ9.HZVCzZwf_qMplGN7UUAqC4FVUsXCi5OINb8yaY7XsHM"
        )
    }
}
