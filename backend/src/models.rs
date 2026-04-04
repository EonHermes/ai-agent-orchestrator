use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub endpoint_url: String,
    pub capabilities: Vec<String>,
    pub status: AgentStatus,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentStatus {
    Active,
    Inactive,
    Error,
}

impl AgentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Error => "error",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub user_query: String,
    pub parsed_plan: Option<serde_json::Value>,
    pub status: TaskStatus,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    Dispatched,
    Completed,
    Failed,
    Cancelled,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Dispatched => "dispatched",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubTask {
    pub id: String,
    pub task_id: String,
    pub agent_id: String,
    pub capability: String,
    pub input: Option<serde_json::Value>,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub status: SubTaskStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub latency_ms: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SubTaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl SubTaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Failed => "failed",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Execution {
    pub id: String,
    pub task_id: String,
    pub agent_id: Option<String>,
    pub step: i32,
    pub action: String,
    pub input_snapshot: Option<serde_json::Value>,
    pub output_snapshot: Option<serde_json::Value>,
    pub latency_ms: Option<i32>,
    pub success: bool,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPerformance {
    pub id: String,
    pub agent_id: String,
    pub capability: String,
    pub success_count: i64,
    pub failure_count: i64,
    pub avg_latency_ms: Option<f64>,
    pub last_updated: DateTime<Utc>,
}

// API Request/Response structs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentRequest {
    pub name: String,
    pub description: Option<String>,
    pub endpoint_url: String,
    pub capabilities: Vec<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAgentRequest {
    pub description: Option<String>,
    pub endpoint_url: Option<String>,
    pub capabilities: Option<Vec<String>>,
    pub status: Option<AgentStatus>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub user_query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResponse {
    pub task: Task,
    pub sub_tasks: Vec<SubTask>,
    pub agent_assignments: Vec<AgentAssignment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAssignment {
    pub sub_task_id: String,
    pub agent_id: String,
    pub agent_name: String,
    pub capability: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseRequest {
    pub user_query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResponse {
    pub plan: Vec<PlanStep>,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    pub capability: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub total_agents: usize,
    pub active_agents: usize,
    pub total_tasks: usize,
    pub active_tasks: usize,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
    pub avg_task_latency_ms: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatsResponse {
    pub agent: Agent,
    pub total_executions: usize,
    pub success_rate: f64,
    pub avg_latency_ms: f64,
    pub top_capabilities: Vec<(String, i64)>,  // (capability, count)
}