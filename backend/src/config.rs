use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub llm: LlmConfig,
    pub task: TaskConfig,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LlmConfig {
    pub openrouter_api_key: String,
    pub openrouter_model: String,
    pub llm_timeout_seconds: u64,
    pub llm_max_tokens: u32,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TaskConfig {
    pub max_concurrent_tasks: usize,
    pub task_timeout_seconds: u64,
    pub enable_learning: bool,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::default())?;
        Ok(cfg.try_into()?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                port: 8081,
                host: "0.0.0.0".to_string(),
            },
            database: DatabaseConfig {
                url: "sqlite:/data/orchestrator.db".to_string(),
            },
            llm: LlmConfig {
                openrouter_api_key: String::new(),
                openrouter_model: "anthropic/claude-3-opus".to_string(),
                llm_timeout_seconds: 60,
                llm_max_tokens: 2000,
            },
            task: TaskConfig {
                max_concurrent_tasks: 10,
                task_timeout_seconds: 300,
                enable_learning: true,
            },
        }
    }
}