use std::env;
use std::net::SocketAddr;

use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use actix_cors::Cors;
use tracing_subscriber::{EnvFilter, fmt};

mod config;
mod db;
mod models;
mod handlers;
mod search;
mod auth;
mod middleware;

use config::AppConfig;
use db::DbPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // Initialize logger
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Load configuration
    let config = AppConfig::from_env().expect("Failed to load configuration");

    // Initialize database
    let pool = DbPool::new(&config.database_url).await.expect("Failed to connect to database");

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to run migrations");

    // Initialize search index
    let search_index = search::SearchIndex::new(&config.search_index_path).await.expect("Failed to initialize search index");

    // Bind address
    let addr = format!("{}:{}", config.host, config.port);
    let addr: SocketAddr = addr.parse().expect("Invalid bind address");

    tracing::info!("Starting server on {}", addr);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(actix_web::http::header::CONTENT_TYPE);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(search_index.clone()))
            .app_data(web::Data::new(config.clone()))
            .configure(handlers::config_routes)
            .default_service(web::route().to(|| async { HttpResponse::NotFound().body("Not Found") }))
    })
    .bind(addr)?
    .run()
    .await
}