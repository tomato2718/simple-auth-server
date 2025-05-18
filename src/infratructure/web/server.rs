use super::api::healthz;
use actix_web::{App, HttpServer};

pub async fn start_server(host: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(healthz))
        .bind((host, port))?
        .run()
        .await
}
