use actix_web::{HttpResponse, Responder, Scope, get, web};

pub fn scope(path: &str) -> Scope {
    web::scope(path).service(healthz)
}

#[get("")]
async fn healthz() -> impl Responder {
    HttpResponse::NoContent()
}
