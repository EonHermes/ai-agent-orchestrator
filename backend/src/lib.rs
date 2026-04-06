use actix_web::test::{call_service, init_service, TestRequest};
use sqlx::PgPool;
use std::env;

use crate::models::{CreateUser, LoginCredentials, CreateSnippet};

mod integration;

#[actix_rt::test]
async fn test_health_check() {
    let pool = setup_test_db().await;
    let app = test_util::setup_app(pool).await;

    let req = TestRequest::get().uri("/health").to_request();
    let resp = call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn test_register_user() {
    let pool = setup_test_db().await;
    let app = test_util::setup_app(pool).await;

    let user_data = CreateUser {
        username: "testuser".to_string(),
        password: "password123".to_string(),
    };

    let req = TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&user_data)
        .to_request();

    let resp = call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
}

async fn setup_test_db() -> PgPool {
    let database_url = env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/snippets_test".to_string());

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    // Clean up before each test
    sqlx::query!("DELETE FROM users")
        .execute(&pool)
        .await
        .ok();

    pool
}

mod test_util {
    use super::*;
    use crate::{handlers::config_routes, config::AppConfig};

    pub async fn setup_app(pool: PgPool) -> actix_web::dev::App {
        let config = AppConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            database_url: "postgres://postgres:postgres@localhost:5432/snippets_test".to_string(),
            search_index_path: "./test_search_index".to_string(),
            jwt_secret: "test-secret-key-for-testing-only".to_string(),
            jwt_expiry_seconds: 3600,
        };

        let search_index = crate::search::SearchIndex::new(&config.search_index_path)
            .await
            .expect("Failed to create search index");

        actix_web::test::init_service(
            crate::handlers::config_routes(
                &mut actix_web::test::TestServer::new().await
                    .app_data(actix_web::web::Data::<PgPool>::new(pool))
                    .app_data(actix_web::web::Data::<crate::search::SearchIndex>::new(search_index))
                    .app_data(actix_web::web::Data::<AppConfig>::new(config))
                    .configure(config_routes)
            )
        ).await
    }
}