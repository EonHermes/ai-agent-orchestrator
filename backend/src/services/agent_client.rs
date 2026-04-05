use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{error, warn};
use std::time::{Duration, Instant};

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Agent returned error: {status} - {body}")]
    AgentError { status: StatusCode, body: String },
    #[error("Timeout after {0:?}")]
    Timeout(Duration),
    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] serde_json::Error),
    #[error("Endpoint not configured")]
    NoEndpoint,
}

#[derive(Debug, Serialize)]
pub struct AgentRequest {
    pub capability: String,
    pub input: serde_json::Value,
    pub context: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct AgentResponse {
    pub success: bool,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

pub struct AgentClient {
    client: Client,
    timeout: Duration,
}

impl AgentClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(60))
                .connect_timeout(Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
            timeout: Duration::from_secs(60),
        }
    }

    pub async fn call_agent(
        &self,
        endpoint_url: &str,
        capability: &str,
        input: serde_json::Value,
        context: Option<serde_json::Value>,
    ) -> Result<AgentResponse, AgentError> {
        if endpoint_url.is_empty() {
            return Err(AgentError::NoEndpoint);
        }

        let start = Instant::now();
        let request = AgentRequest {
            capability: capability.to_string(),
            input,
            context,
        };

        let response = self
            .client
            .post(endpoint_url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let latency = start.elapsed();

        if !response.status().is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(AgentError::AgentError {
                status: response.status(),
                body,
            });
        }

        let agent_response: AgentResponse = response.json().await?;
        Ok(agent_response)
    }
}
