use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: i32,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: i32,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub priority: Option<i32>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskFilter {
    pub status: Option<TaskStatus>,
    pub priority_min: Option<i32>,
    pub priority_max: Option<i32>,
    pub tags: Vec<String>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub r#type: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskBroadcast {
    pub r#type: String,
    pub task: Task,
    pub timestamp: DateTime<Utc>,
}

impl TaskBroadcast {
    pub fn created(task: Task) -> Self {
        Self {
            r#type: "task_created".to_string(),
            task,
            timestamp: Utc::now(),
        }
    }

    pub fn updated(task: Task) -> Self {
        Self {
            r#type: "task_updated".to_string(),
            task,
            timestamp: Utc::now(),
        }
    }

    pub fn deleted(task_id: Uuid) -> Self {
        Self {
            r#type: "task_deleted".to_string(),
            task: Task {
                id: task_id,
                title: "".to_string(),
                description: None,
                status: TaskStatus::Todo,
                priority: 0,
                tags: vec![],
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            timestamp: Utc::now(),
        }
    }
}
