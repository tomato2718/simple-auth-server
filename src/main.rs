use simple_auth_server::start_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start_server("0.0.0.0", 8080).await
}
