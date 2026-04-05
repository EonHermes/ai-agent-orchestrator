use axum::{
    extract::{State, Json, Extension},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use crate::{db::Repository, services::llm::LlmService};

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub database: String,
    pub timestamp: String,
}

pub async fn health_check(State(repo): State<Repository>) -> (StatusCode, Json<HealthResponse>) {
    // Check database connectivity
    let db_status = match sqlx::query("SELECT 1").execute(&repo.pool).await {
        Ok(_) => "healthy",
        Err(e) => {
            eprintln!("Database health check failed: {}", e);
            "unhealthy"
        }
    };

    let overall_status = if db_status == "healthy" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (
        overall_status,
        Json(HealthResponse {
            status: if db_status == "healthy" { "healthy" } else { "degraded" }.to_string(),
            database: db_status.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }),
    )
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub agent_count: usize,
    pub task_stats: TaskStats,
    pub uptime_seconds: u64,
    pub start_time: String,
}

#[derive(Serialize)]
pub struct TaskStats {
    pub pending: usize,
    pub dispatched: usize,
    pub completed: usize,
    pub failed: usize,
    pub total: usize,
}

pub async fn status(State(repo): State<Repository>) -> (StatusCode, Json<StatusResponse>) {
    let agents = match repo.list_agents(None).await {
        Ok(agents) => agents,
        Err(e) => {
            eprintln!("Error fetching agents: {}", e);
            Vec::new()
        }
    };

    let tasks = match repo.list_tasks(None, None, None, None).await {
        Ok(tasks) => tasks,
        Err(e) => {
            eprintln!("Error fetching tasks: {}", e);
            Vec::new()
        }
    };

    let mut task_stats = TaskStats {
        pending: 0,
        dispatched: 0,
        completed: 0,
        failed: 0,
        total: tasks.len(),
    };

    for task in tasks {
        match task.status {
            TaskStatus::Pending => task_stats.pending += 1,
            TaskStatus::Dispatched => task_stats.dispatched += 1,
            TaskStatus::Completed => task_stats.completed += 1,
            TaskStatus::Failed => task_stats.failed += 1,
            TaskStatus::Cancelled => {}
        }
    }

    (
        StatusCode::OK,
        Json(StatusResponse {
            agent_count: agents.len(),
            task_stats,
            uptime_seconds: 0, // Would need startup timestamp tracking
            start_time: chrono::Utc::now().to_rfc3339(),
        }),
    )
}

#[derive(Deserialize)]
pub struct ParseOnlyRequest {
    pub user_query: String,
}

#[derive(Serialize)]
pub struct ParseOnlyResponse {
    pub success: bool,
    pub parsed: Option<Vec<crate::services::llm::ParseResult>>,
    pub error: Option<String>,
}

pub async fn parse_only(
    State(repo): State<Repository>,
    Extension(llm): Extension<Arc<LlmService>>,
    Json(req): Json<ParseOnlyRequest>,
) -> (StatusCode, Json<ParseOnlyResponse>) {
    match llm.parse_task(&req.user_query).await {
        Ok(parsed) => (
            StatusCode::OK,
            Json(ParseOnlyResponse {
                success: true,
                parsed: Some(parsed),
                error: None,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ParseOnlyResponse {
                success: false,
                parsed: None,
                error: Some(e.to_string()),
            }),
        ),
    }
}
