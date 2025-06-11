use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{HttpResponse, Responder, Scope, post, web};
use serde::Deserialize;

use crate::application::use_case::{CreateUserDTO, SignInUseCase, SignUpUseCase};
use crate::domain::{entity::User, repository::UserRepository, value_object::EmailAddress};
use crate::infratructure::{
    auth::{BcryptHasher, BcryptValidator, InfraClaims, JWTIssuer},
    repository::{GenericTableManager, InMemoryUserRepository},
    system::get_systime,
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

#[post("/signin")]
async fn signin(body: web::Json<SignInRequestBody>) -> impl Responder {
    // let sign_in = SignInUseCase::new(
    //     &BcryptValidator {},
    //     &JWTIssuer::new(
    //         b"",
    //         InfraClaims {
    //             iss: "test".to_string(),
    //             aud: "test".to_string(),
    //             iat: 123,
    //             exp: 123,
    //         },
    //     ),
    //     &JWTIssuer::new(
    //         b"",
    //         InfraClaims {
    //             iss: "test".to_string(),
    //             aud: "test".to_string(),
    //             iat: 123,
    //             exp: 123,
    //         },
    //     ),
    //     &InMemoryUserRepository::new(),
    // );

    HttpResponse::NoContent()
}
