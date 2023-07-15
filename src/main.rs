mod config;
mod handler;
mod middleware;
mod response;

mod abstract_trait;
mod models;
mod repository;
mod schema;
mod service;
mod service_register;

use crate::service_register::ServiceRegister;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{http::header, App, HttpServer};
use config::{Config, ConnectionManager};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();
    let config = Config::init();

    let db_pool =
        match ConnectionManager::new_pool(&config.database_url, config.run_migrations).await {
            Ok(pool) => pool,
            Err(err) => {
                eprintln!("Error initializing database connection pool: {}", err);
                return Ok(());
            }
        };

    let port = config.port;
    let service_register = ServiceRegister::new(db_pool, config);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .configure(handler::config)
            .app_data(Data::new(service_register.clone()))
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
