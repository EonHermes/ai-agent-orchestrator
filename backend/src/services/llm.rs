use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{error, info};
use crate::config::LlmConfig;

#[derive(Debug, Error)]
pub enum LlmError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("API error: {status} - {message}")]
    ApiError { status: StatusCode, message: String },
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: MessageContent,
}

#[derive(Debug, Deserialize)]
pub struct MessageContent {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningRequest {
    pub user_query: String,
    pub available_agents: Vec<AgentInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentInfo {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsingRequest {
    pub user_query: String,
}

pub struct LlmService {
    client: Client,
    config: LlmConfig,
}

impl LlmService {
    pub fn new(config: LlmConfig) -> Result<Self, LlmError> {
        if config.openrouter_api_key.is_empty() {
            return Err(LlmError::ConfigError("OpenRouter API key is required".to_string()));
        }

        Ok(Self {
            client: Client::new(),
            config,
        })
    }

    pub async fn parse_task(&self, user_query: &str) -> Result<Vec<ParseResult>, LlmError> {
        let system_prompt = r#"
You are an AI task parser. Analyze user queries and extract structured action items.
Return a JSON array of objects with:
- "action": the verb/noun describing the action (e.g., "analyze_data", "generate_report")
- "description": detailed description of what needs to be done
- "inputs": array of required input parameters
- "outputs": array of expected output parameters
- "complexity": "low", "medium", or "high"

Example:
Query: "Analyze last week's model performance and create a PDF report"
Response:
[
  {
    "action": "query_model_runs",
    "description": "Retrieve model training runs from last week",
    "inputs": ["time_range", "model_version"],
    "outputs": ["training_metrics", "performance_data"],
    "complexity": "medium"
  },
  {
    "action": "analyze_metrics",
    "description": "Compute statistics and trends from performance data",
    "inputs": ["performance_data", "metrics_list"],
    "outputs": ["statistical_summary", "recommendations"],
    "complexity": "high"
  },
  {
    "action": "generate_report",
    "description": "Create PDF report with charts and recommendations",
    "inputs": ["statistical_summary", "template"],
    "outputs": ["pdf_report"],
    "complexity": "medium"
  }
]
Only return the JSON array, nothing else.
"#;

        let user_prompt = format!("Parse this query: {}", user_query);

        let request = ChatCompletionRequest {
            model: self.config.openrouter_model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_prompt,
                },
            ],
            temperature: self.config.parsing_temperature,
            max_tokens: Some(2000),
        };

        let response: ChatCompletionResponse = self
            .client
            .post(&self.config.openrouter_endpoint)
            .header("Authorization", format!("Bearer {}", self.config.openrouter_api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        let content = response
            .choices
            .first()
            .ok_or_else(|| LlmError::ApiError {
                status: StatusCode::NO_CONTENT,
                message: "No choices returned".to_string(),
            })?
            .message
            .content
            .clone();

        serde_json::from_str(&content).map_err(|e| LlmError::JsonError(e))
    }

    pub async fn create_execution_plan(
        &self,
        user_query: &str,
        available_agents: Vec<AgentInfo>,
    ) -> Result<Vec<PlanStep>, LlmError> {
        let agents_json = serde_json::to_string_pretty(&available_agents).unwrap_or_default();

        let system_prompt = format!(
            r#"
You are an AI execution planner. Given a user query and available agents, create an optimal execution plan.
Map each action from the parsed query to an appropriate agent based on capabilities.
Return a JSON array of plan steps in order of execution.

Each step must include:
- "agent_id": the ID of the agent to use
- "capability": the capability to invoke
- "input": an object with the parameters for that agent
- "order": sequential number (0, 1, 2...)

Important:
- Consider dependencies: later steps may need output from earlier steps
- Don't assign tasks to agents that lack the required capability
- If no agent has a needed capability, assign "null" agent_id and explain why
- Preserve data flow: output from step N should be available as input to step N+1

Available Agents:
{}

Return ONLY the JSON array.
"#,
            agents_json
        );

        let user_prompt = format!("Create an execution plan for: {}", user_query);

        let request = ChatCompletionRequest {
            model: self.config.openrouter_model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_prompt,
                },
            ],
            temperature: self.config.planning_temperature,
            max_tokens: Some(4000),
        };

        let response: ChatCompletionResponse = self
            .client
            .post(&self.config.openrouter_endpoint)
            .header("Authorization", format!("Bearer {}", self.config.openrouter_api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        let content = response
            .choices
            .first()
            .ok_or_else(|| LlmError::ApiError {
                status: StatusCode::NO_CONTENT,
                message: "No choices returned".to_string(),
            })?
            .message
            .content
            .clone();

        serde_json::from_str(&content).map_err(|e| LlmError::JsonError(e))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParseResult {
    pub action: String,
    pub description: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub complexity: String,
}
