use axum::{Router, Server};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use sqlx::{SqlitePool, migrate::MigrateDatabase};
use std::net::SocketAddr;
use std::sync::Arc;

mod config;
mod models;
mod db;
mod llm_service;
mod agent_service;
mod task_service;
mod handlers;

use config::Config;
use llm_service::LlmService;
use agent_service::AgentService;
use task_service::TaskService;
use handlers::*;

pub type SharedState = Arc<AppState>;

struct AppState {
    config: Config,
    db_pool: SqlitePool,
    agent_service: AgentService,
    llm_service: LlmService,
    task_service: TaskService,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Starting AI Agent Orchestrator...");

    // Load configuration
    let config = Config::from_env().map_err(|e| {
        anyhow::anyhow!("Failed to load config from environment: {}", e)
    })?;

    // Initialize database
    let db_url = config.database.url.clone();

    // Create database if it doesn't exist
    if !sqlx::Sqlite::database_exists(&db_url).await? {
        info!("Creating database at {}", db_url);
        sqlx::Sqlite::create_database(&db_url).await?;
    }

    let db_pool = SqlitePool::connect_lazy(&db_url)?;
    info!("Connected to database");

    // Run migrations
    db::run_migrations(&db_pool).await?;
    info!("Database migrations complete");

    // Initialize services
    let llm_service = LlmService::new(
        config.llm.openrouter_api_key.clone(),
        config.llm.openrouter_model.clone(),
        config.llm.llm_timeout_seconds,
        config.llm.llm_max_tokens,
    );

    let agent_service = AgentService::new(db_pool.clone(), llm_service.clone());
    let task_service = TaskService::new(
        db_pool.clone(),
        llm_service.clone(),
        agent_service.clone(),
        config.task.max_concurrent_tasks,
    );

    let shared_state = Arc::new(AppState {
        config: config.clone(),
        db_pool: db_pool.clone(),
        agent_service,
        llm_service,
        task_service,
    });

    // Build routes
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/api/v1/status", get(status_handler))
        // Agents
        .route("/api/v1/agents", get(list_agents_handler).post(create_agent_handler))
        .route("/api/v1/agents/:id", get(get_agent_handler).put(update_agent_handler).delete(delete_agent_handler))
        .route("/api/v1/agents/capabilities", get(list_capabilities_handler))
        .route("/api/v1/agents/:id/stats", get(get_agent_stats_handler))
        // Tasks
        .route("/api/v1/tasks", post(create_task_handler).get(list_tasks_handler))
        .route("/api/v1/tasks/:id", get(get_task_handler).post(cancel_task_handler))
        .route("/api/v1/tasks/:id/plan", get(get_task_plan_handler))
        .route("/api/v1/parse", post(parse_task_handler))
        // Executions
        .route("/api/v1/executions", get(list_executions_handler))
        .route("/api/v1/executions/:id", get(get_execution_handler))
        .route("/api/v1/executions/stats", get(get_execution_stats_handler))
        // Agent endpoint (for receiving tasks from orchestrator)
        .route("/agent/execute", post(agent_execute_handler))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(shared_state);

    // Bind and serve
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    info!("Listening on http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}