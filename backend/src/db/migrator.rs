use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::path::Path;

pub async fn init_database(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // Create database file if it doesn't exist
    if database_url.starts_with("sqlite:") && !database_url.ends_with(":memory:") {
        let path = database_url.trim_start_matches("sqlite:");
        if let Some(parent) = Path::new(path).parent() {
            std::fs::create_dir_all(parent).expect("Failed to create database directory");
        }
    }

    // Run migrations
    sqlx::Sqlite::create(database_url).await?;

    let pool = SqlitePool::connect(database_url).await?;

    // Enable WAL mode for better concurrency
    sqlx::query("PRAGMA journal_mode=WAL").execute(&pool).await?;
    sqlx::query("PRAGMA synchronous=NORMAL").execute(&pool).await?;
    sqlx::query("PRAGMA cache_size=1000").execute(&pool).await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
