use actix_web::{HttpResponse, Responder, get};

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::NoContent()
}
