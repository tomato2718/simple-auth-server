use actix_web::{HttpResponse, Responder, Scope, post, web};
use serde::Deserialize;

use crate::application::use_case::{CreateUserDTO, SignInUseCase};
use crate::infratructure::{
    auth::{BcryptHasher, BcryptValidator, JWTIssuer},
    repository::InMemoryUserRepository,
};

pub fn scope(path: &str) -> Scope {
    web::scope(path).service(signin).service(signup)
}

#[derive(Deserialize)]
struct SignInRequestBody {
    email: String,
    password: String,
}

#[post("/signup")]
async fn signup() -> impl Responder {
    HttpResponse::NoContent()
}

#[post("/signin")]
async fn signin(body: web::Json<SignInRequestBody>) -> impl Responder {
    HttpResponse::NoContent()
}
