use std::net::SocketAddr;
use std::sync::Arc;
use task_dashboard_backend::{api, db, error::Result};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "task_dashboard_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Arc::new(config::Config::from_env()?);

    // Connect to database
    let db_pool = db::connect(&config).await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&db_pool).await?;

    tracing::info!("Database migrations applied successfully");

    // Build application routes
    let app = api::router(db_pool, config);

    // Bind and serve
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| error::Error::Io(e).into())

}
