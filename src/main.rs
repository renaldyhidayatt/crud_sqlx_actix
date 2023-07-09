use std::sync::Arc;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{http::header, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::handler::health_checker_handler;
use crate::repository::NoteRepository;
use crate::service::NoteService;

mod handler;
mod model;
mod repository;
mod schema;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let note_repository = NoteRepository {
        db_pool: db_pool.clone(),
    };
    let shared_repository: Arc<NoteRepository> = Arc::new(note_repository);
    let note_service = NoteService::new(shared_repository.clone());

    println!("🚀 Server started successfully");

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
            .app_data(Data::new(Arc::new(note_service.clone())))
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
