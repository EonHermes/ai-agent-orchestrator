mod config;
mod db;
mod models;
mod services;
mod handlers;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::Settings;
use db::{migrator, Repository};
use services::{agent_client::AgentClient, executor::Executor, llm::LlmService};
use handlers::create_routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let settings = Settings::new().unwrap_or_else(|_| Settings::default());

    // Initialize logging
    let filter_level = match settings.logging.level.as_str() {
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("orchestrator={}", filter_level.to_string())),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting AI Agent Orchestrator v1.0.0");
    info!("Configuration loaded: port={}, database={}", settings.server.port, settings.database.url);

    // Initialize database
    info!("Initializing database...");
    let pool = migrator::init_database(&settings.database.url).await?;
    migrator::run_migrations(&pool).await?;
    info!("Database initialized and migrations applied");

    // Initialize services
    let repo = Repository::new(pool);
    let llm = Arc::new(LlmService::new(settings.llm)?);
    let agent_client = Arc::new(AgentClient::new());
    let executor = Arc::new(Executor::new(
        Arc::new(repo.clone()),
        llm.clone(),
        agent_client.clone(),
        settings.server.max_concurrent_tasks,
    ));

    // Build application
    let app = create_routes(repo.clone(), llm, executor)
        .layer(CorsLayer::new().allow_origin(
            settings
                .cors
                .allowed_origins
                .iter()
                .next()
                .unwrap_or(&"http://localhost:3000".to_string())
                .parse()?,
        ).allow_credentials(settings.cors.allow_credentials))
        .layer(TraceLayer::new_for_http());

    // Determine address
    let addr = SocketAddr::from((
        settings.server.host.parse().unwrap_or([0, 0, 0, 0].into()),
        settings.server.port,
    ));

    info!("Listening on http://{}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
