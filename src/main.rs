use std::sync::Arc;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{http::header, App, HttpServer};
use config::Config;
use dotenv::dotenv;

use repository::{NoteRepository, UserRepository};
use service::{NoteService, UserService};

use sqlx::postgres::PgPoolOptions;

mod config;
mod handler;
mod middleware;
mod response;

mod models;
mod repository;
mod schema;
mod service;

pub struct AppState {
    pub user_service: Arc<UserService>,
    pub note_service: Arc<NoteService>,
    pub env: Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();
    let config = Config::init();

    let db_pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
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
    let note_repository_shared: Arc<NoteRepository> = Arc::new(note_repository);
    let note_service = NoteService::new(note_repository_shared.clone());

    let user_repository = UserRepository {
        db_pool: db_pool.clone(),
    };
    let user_repository_shared: Arc<UserRepository> = Arc::new(user_repository);
    let user_service = UserService::new(user_repository_shared.clone());

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
            .app_data(Data::new(AppState {
                env: config.clone(),
                user_service: Arc::new(user_service.clone()),
                note_service: Arc::new(note_service.clone()),
            }))
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
