use super::scope::{auth, healthz};
use crate::{
    domain::entity::User,
    infratructure::{repository::GenericTableManager, system::get_envvar},
};
use actix_web::{App, HttpServer, web};

pub async fn start_server(host: &str, port: u16) -> std::io::Result<()> {
    let envvar = web::Data::new(get_envvar());
    let user_table_manager = web::Data::new(GenericTableManager::<User>::new());
    HttpServer::new(move || {
        App::new()
            .app_data(envvar.clone())
            .app_data(user_table_manager.clone())
            .service(healthz::scope("/healthz"))
            .service(auth::scope(""))
    })
    .bind((host, port))?
    .run()
    .await
}
