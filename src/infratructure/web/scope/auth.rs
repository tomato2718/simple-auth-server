use actix_web::{HttpResponse, Responder, Scope, http::header::ContentType, post, web};
use serde::{Deserialize, Serialize};

use crate::application::use_case::{CreateUserDTO, SignInUseCase, SignUpUseCase};
use crate::domain::{entity::User, value_object::EmailAddress};
use crate::infratructure::{
    auth::{BcryptHasher, BcryptValidator, InfraClaims, JWTIssuer},
    repository::{GenericTableManager, InMemoryUserRepository},
    system::{EnvVar, get_systime},
};

pub fn scope(path: &str) -> Scope {
    web::scope(path).service(signin).service(signup)
}

#[derive(Deserialize)]
struct SignUpRequestBody {
    email: String,
    username: String,
    password: String,
}

#[post("/signup")]
async fn signup(
    body: web::Json<SignUpRequestBody>,
    user_inmemory_table: web::Data<GenericTableManager<User>>,
) -> impl Responder {
    let password_hasher = BcryptHasher::new(12);
    let mut user_repository = InMemoryUserRepository::new(user_inmemory_table.get_table());
    let mut sign_up = SignUpUseCase::new(&password_hasher, &mut user_repository, get_systime);

    let email = match EmailAddress::new(&body.email) {
        Ok(email) => email,
        Err(_) => return HttpResponse::UnprocessableEntity(),
    };
    let result = sign_up.execute(CreateUserDTO {
        email_address: email,
        username: body.username.clone(),
        password: body.password.clone(),
    });

    match result {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::Conflict(),
    }
}

#[derive(Deserialize)]
struct SignInRequestBody {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct SignInResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub username: String,
    pub email: String,
}

#[post("/signin")]
async fn signin(
    body: web::Json<SignInRequestBody>,
    user_inmemory_table: web::Data<GenericTableManager<User>>,
    envvar: web::Data<EnvVar>,
) -> HttpResponse {
    let user_repository = InMemoryUserRepository::new(user_inmemory_table.get_table());
    let now = get_systime();
    let access_token_issuer = JWTIssuer::new(
        &envvar.access_token_secret,
        InfraClaims {
            iss: envvar.app_name.clone(),
            aud: envvar.app_name.clone(),
            iat: now,
            exp: now + envvar.access_token_valid_seconds,
        },
    );
    let refresh_token_issuer = JWTIssuer::new(
        &envvar.refresh_token_secret,
        InfraClaims {
            iss: envvar.app_name.clone(),
            aud: envvar.app_name.clone(),
            iat: now,
            exp: now + envvar.refresh_token_valid_seconds,
        },
    );
    let sign_in = SignInUseCase::new(
        &BcryptValidator {},
        &access_token_issuer,
        &refresh_token_issuer,
        &user_repository,
    );
    let email = match EmailAddress::new(&body.email) {
        Ok(email) => email,
        Err(_) => return HttpResponse::UnprocessableEntity().finish(),
    };

    let result = sign_in.execute(email, &body.password);

    match result {
        Ok(res) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(SignInResponse {
                access_token: res.access_token,
                refresh_token: res.refresh_token,
                username: res.username,
                email: res.email,
            }),
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}
