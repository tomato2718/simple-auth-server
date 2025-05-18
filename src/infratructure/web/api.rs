use actix_web::{HttpResponse, Responder, get};

#[get("/healthz")]
pub async fn healthz() -> impl Responder {
    HttpResponse::NoContent()
}
