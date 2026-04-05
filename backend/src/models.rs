use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AgentStatus {
    Active,
    Inactive,
    Error,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewAgent {
    pub name: String,
    pub description: Option<String>,
    pub endpoint_url: String,
    pub capabilities: Vec<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserTask {
    pub id: String,
    pub user_query: String,
    pub parsed_plan: Option<serde_json::Value>,
    pub status: TaskStatus,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    Dispatched,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewTask {
    pub user_query: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub latency_ms: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SubTaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewSubTask {
    pub task_id: String,
    pub agent_id: String,
    pub capability: String,
    pub input: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Execution {
    pub id: String,
    pub task_id: String,
    pub agent_id: Option<String>,
    pub step: i64,
    pub action: String,
    pub input_snapshot: Option<serde_json::Value>,
    pub output_snapshot: Option<serde_json::Value>,
    pub latency_ms: Option<i64>,
    pub success: bool,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentPerformance {
    pub id: String,
    pub agent_id: String,
    pub capability: String,
    pub success_count: i64,
    pub failure_count: i64,
    pub avg_latency_ms: Option<f64>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParsedPlan {
    pub steps: Vec<PlanStep>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanStep {
    pub agent_id: String,
    pub capability: String,
    pub input: serde_json::Value,
    pub order: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskSubmission {
    pub user_query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub task: UserTask,
    pub sub_tasks: Vec<SubTask>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentListResponse {
    pub agents: Vec<Agent>,
    pub total: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionStats {
    pub total_executions: i64,
    pub success_rate: f64,
    pub avg_latency_ms: f64,
    pub by_agent: Vec<AgentStats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentStats {
    pub agent_id: String,
    pub agent_name: String,
    pub total_tasks: i64,
    pub success_rate: f64,
    pub avg_latency_ms: f64,
}

impl Agent {
    pub fn new(
        name: String,
        description: Option<String>,
        endpoint_url: String,
        capabilities: Vec<String>,
        metadata: Option<serde_json::Value>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            endpoint_url,
            capabilities,
            status: AgentStatus::Active,
            metadata,
            created_at: now,
            updated_at: now,
        }
    }
}

impl UserTask {
    pub fn new(user_query: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            user_query,
            parsed_plan: None,
            status: TaskStatus::Pending,
            error_message: None,
            created_at: now,
            started_at: None,
            completed_at: None,
        }
    }
}

impl SubTask {
    pub fn new(task_id: String, agent_id: String, capability: String, input: Option<serde_json::Value>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            task_id,
            agent_id,
            capability,
            input,
            output: None,
            error: None,
            status: SubTaskStatus::Pending,
            started_at: None,
            completed_at: None,
            latency_ms: None,
        }
    }
}

impl Execution {
    pub fn new(
        task_id: String,
        agent_id: Option<String>,
        step: i64,
        action: String,
        input_snapshot: Option<serde_json::Value>,
        output_snapshot: Option<serde_json::Value>,
        latency_ms: Option<i64>,
        success: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            task_id,
            agent_id,
            step,
            action,
            input_snapshot,
            output_snapshot,
            latency_ms,
            success,
            timestamp: Utc::now(),
        }
    }
}
