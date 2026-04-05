use config::{Config, Environment, File};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub llm: LlmConfig,
    pub cors: CorsConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_concurrent_tasks: usize,
    pub task_timeout_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LlmConfig {
    pub openrouter_api_key: String,
    pub openrouter_model: String,
    pub openrouter_endpoint: String,
    pub planning_temperature: f32,
    pub parsing_temperature: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allow_credentials: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub format: LogFormat,
}

#[derive(Debug, Deserialize, Clone)]
pub enum LogFormat {
    Json,
    Text,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config_path = if run_mode == "production" {
            "/etc/orchestrator/config"
        } else {
            "config"
        };

        let mut config = Config::new();

        if Path::new(&format!("{}/default.toml", config_path)).exists() {
            config = config.add_source(File::with_name(&format!("{}/default", config_path)));
        }

        if Path::new(&format!("{}/{}.toml", config_path, run_mode)).exists() {
            config = config.add_source(File::with_name(&format!("{}/{}", config_path, run_mode)));
        }

        config = config
            .add_source(Environment::with_prefix("ORCHESTRATOR"))
            .add_source(Environment::default());

        config.try_into()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8081,
                max_concurrent_tasks: 10,
                task_timeout_seconds: 300,
            },
            database: DatabaseConfig {
                url: "sqlite:/data/orchestrator.db".to_string(),
            },
            llm: LlmConfig {
                openrouter_api_key: "".to_string(),
                openrouter_model: "anthropic/claude-3-opus".to_string(),
                openrouter_endpoint: "https://openrouter.ai/api/v1/chat/completions".to_string(),
                planning_temperature: 0.7,
                parsing_temperature: 0.3,
            },
            cors: CorsConfig {
                allowed_origins: vec!["http://localhost:3000".to_string()],
                allow_credentials: true,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: LogFormat::Json,
            },
        }
    }
}
