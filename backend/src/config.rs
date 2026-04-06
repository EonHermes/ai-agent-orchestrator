use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub search_index_path: String,
    pub jwt_secret: String,
    pub jwt_expiry_seconds: u64,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(AppConfig {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a number"),
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:snippets.db".to_string()),
            search_index_path: env::var("SEARCH_INDEX_PATH").unwrap_or_else(|_| "./search_index".to_string()),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            jwt_expiry_seconds: env::var("JWT_EXPIRY_SECONDS")
                .unwrap_or_else(|_| "86400".to_string())
                .parse()
                .expect("JWT_EXPIRY_SECONDS must be a number"),
        })
    }
}