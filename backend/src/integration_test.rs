#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test::{call_service, init_service, TestRequest}};
    use sqlx::PgPool;
    use std::env;

    use crate::{
        config::AppConfig,
        db::DbPool,
        handlers::config_routes,
        search::SearchIndex,
    };

    async fn setup_app(pool: PgPool) -> actix_web::dev::App {
        let config = AppConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            database_url: "postgres://postgres:postgres@localhost:5432/snippets_test".to_string(),
            search_index_path: "./test_search_index".to_string(),
            jwt_secret: "test-secret".to_string(),
            jwt_expiry_seconds: 3600,
        };

        let search_index = SearchIndex::new(&config.search_index_path)
            .await
            .expect("Failed to create search index");

        init_service(
            config_routes(
                &mut actix_web::test::TestServer::new().await
                    .app_data(actix_web::web::Data::new(pool))
                    .app_data(actix_web::web::Data::new(search_index))
                    .app_data(actix_web::web::Data::new(config))
                    .configure(config_routes)
            )
        ).await
    }

    #[actix_rt::test]
    async fn test_health_check() {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/snippets_test".to_string());

        let pool = PgPool::connect(&database_url).await.unwrap();
        let app = setup_app(pool).await;

        let req = TestRequest::get().uri("/health").to_request();
        let resp = call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}