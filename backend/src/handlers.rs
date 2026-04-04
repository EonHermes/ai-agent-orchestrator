use axum::{
    routing::{get, post, put, delete},
    extract::{State, Json, Path, Query},
    http::{StatusCode, header},
    response::{IntoResponse, Redirect, Response},
};
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::models::*;
use crate::db;
use crate::config::Config;
use crate::agent_service::AgentService;
use crate::llm_service::LlmService;
use crate::task_service::TaskService;
use sqlx::SqlitePool;

pub struct AppState {
    pub config: Config,
    pub db_pool: SqlitePool,
    pub agent_service: AgentService,
    pub llm_service: LlmService,
    pub task_service: TaskService,
}

// Health check
pub async fn health_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // Check database connectivity
    let db_ok = sqlx::query("SELECT 1").execute(&state.db_pool).await.is_ok();

    let status = if db_ok { "healthy" } else { "unhealthy" };
    let code = if db_ok { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };

    (code, Json(serde_json::json!({ "status": status, "timestamp": chrono::Utc::now() })))
}

// Get system status
pub async fn status_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let agents = db::list_agents(&state.db_pool, None).await.unwrap_or_default();
    let active_agents = agents.iter()
        .filter(|a| a.get("status").and_then(|s| s.as_str()) == Some("active"))
        .count();

    let tasks = db::list_tasks(&state.db_pool, None, None, None).await.unwrap_or_default();
    let active_tasks = tasks.iter()
        .filter(|t| {
            let status = t.get("status").and_then(|s| s.as_str());
            matches!(status, Some("pending" | "dispatched"))
        })
        .count();
    let completed_tasks = tasks.iter()
        .filter(|t| t.get("status").and_then(|s| s.as_str()) == Some("completed"))
        .count();
    let failed_tasks = tasks.iter()
        .filter(|t| t.get("status").and_then(|s| s.as_str()) == Some("failed"))
        .count();

    // Calculate average task latency from executions
    let avg_latency: Option<f64> = sqlx::query_scalar(
        "SELECT AVG(latency_ms) FROM executions WHERE latency_ms IS NOT NULL"
    )
    .fetch_one(&state.db_pool)
    .await
    .ok()
    .flatten();

    Json(StatusResponse {
        total_agents: agents.len(),
        active_agents,
        total_tasks: tasks.len(),
        active_tasks,
        completed_tasks,
        failed_tasks,
        avg_task_latency_ms: avg_latency,
    })
}

// ============ AGENTS ============

// List agents
pub async fn list_agents_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let status = params.get("status").map(String::as_str);
    match db::list_agents(&state.db_pool, status).await {
        Ok(agents) => Json(agents).into_response(),
        Err(e) => {
            warn!("Failed to list agents: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
        }
    }
}

// Create agent
pub async fn create_agent_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateAgentRequest>,
) -> impl IntoResponse {
    match state.agent_service.register_agent_with_suggestions(
        &req.name,
        req.description.as_deref().unwrap_or_default(),
        &req.endpoint_url,
        req.capabilities,
        req.metadata,
    ).await {
        Ok(id) => {
            info!("Created agent {} with id {}", req.name, id);
            (StatusCode::CREATED, Json(serde_json::json!({ "id": id }))).into_response()
        }
        Err(e) => {
            warn!("Failed to create agent: {}", e);
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
        }
    }
}

// Get agent
pub async fn get_agent_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match db::get_agent(&state.db_pool, &id).await {
        Ok(Some(agent)) => Json(agent).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Agent not found" }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response(),
    }
}

// Update agent
pub async fn update_agent_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateAgentRequest>,
) -> impl IntoResponse {
    match db::update_agent(
        &state.db_pool,
        &id,
        req.description.as_deref(),
        req.endpoint_url.as_deref(),
        req.capabilities.as_ref(),
        req.status.as_ref().map(|s| s.as_str()),
        req.metadata.as_ref(),
    ).await {
        Ok(_) => {
            info!("Updated agent {}", id);
            StatusCode::NO_CONTENT.into_response()
        }
        Err(e) => {
            warn!("Failed to update agent {}: {}", id, e);
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
        }
    }
}

// Delete agent
pub async fn delete_agent_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match db::delete_agent(&state.db_pool, &id).await {
        Ok(_) => {
            info!("Deleted agent {}", id);
            StatusCode::NO_CONTENT.into_response()
        }
        Err(e) => {
            warn!("Failed to delete agent {}: {}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
        }
    }
}

// List capabilities
pub async fn list_capabilities_handler(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match db::list_capabilities(&state.db_pool).await {
        Ok(caps) => Json(caps).into_response(),
        Err(e) => {
            warn!("Failed to list capabilities: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
        }
    }
}

// ============ TASKS ============

// Create and execute task
pub async fn create_task_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    if req.user_query.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "user_query cannot be empty" }))).into_response();
    }

    match state.task_service.create_and_execute_task(&req.user_query).await {
        Ok(task_id) => {
            info!("Created task {} for query: {}", task_id, req.user_query);
            (StatusCode::CREATED, Json(serde_json::json!({ "task_id": task_id }))).into_response()
        }
        Err(e) => {
            error!("Failed to create task: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
        }
    }
}

// List tasks
pub async fn list_tasks_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let status = params.get("status").map(String::as_str);
    let limit = params.get("limit")
        .and_then(|s| s.parse().ok());
    let offset = params.get("offset")
        .and_then(|s| s.parse().ok());

    match db::list_tasks(&state.db_pool, status, limit, offset).await {
        Ok(tasks) => Json(tasks).into_response(),
        Err(e) => {
            warn!("Failed to list tasks: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
        }
    }
}

// Get task with sub-tasks
pub async fn get_task_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.task_service.get_task_status(&id).await {
        Ok(Some(task_data)) => Json(task_data).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Task not found" }))).into_response(),
        Err(e) => {
            warn!("Failed to get task {}: {}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
        }
    }
}

// Cancel task
pub async fn cancel_task_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match db::update_task_status(&state.db_pool, &id, "cancelled", None).await {
        Ok(_) => {
            info!("Cancelled task {}", id);
            StatusCode::NO_CONTENT.into_response()
        }
        Err(e) => {
            warn!("Failed to cancel task {}: {}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
        }
    }
}

// Parse without executing
pub async fn parse_task_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ParseRequest>,
) -> impl IntoResponse {
    if req.user_query.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "user_query cannot be empty" }))).into_response();
    }

    let capabilities = match db::list_capabilities(&state.db_pool).await {
        Ok(caps) => caps,
        Err(e) => {
            warn!("Failed to get capabilities: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response();
        }
    };

    match state.llm_service.parse_task(&req.user_query, &capabilities).await {
        Ok(result) => Json(ParseResponse {
            plan: result.plan,
            reasoning: result.reasoning,
        }).into_response(),
        Err(e) => {
            warn!("Parse failed for query '{}': {}", req.user_query, e);
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
        }
    }
}

// Get task plan
pub async fn get_task_plan_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let task = match db::get_task(&state.db_pool, &id).await {
        Ok(Some(t)) => t,
        Ok(None) => return (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Task not found" }))).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response(),
    };

    if let Some(plan) = task.get("parsed_plan") {
        Json(serde_json::json!({ "plan": plan })).into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "No plan available yet" }))).into_response()
    }
}

// ============ EXECUTIONS ============

// List all execution logs
pub async fn list_executions_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let limit = params.get("limit")
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    let rows = match sqlx::query(
        "SELECT e.*, a.name as agent_name FROM executions e LEFT JOIN agents a ON e.agent_id = a.id ORDER BY e.timestamp DESC LIMIT ?"
    )
    .bind(limit)
    .fetch_all(&state.db_pool)
    .await {
        Ok(rows) => rows,
        Err(e) => {
            warn!("Failed to fetch executions: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response();
        }
    };

    let executions = rows.iter().map(|row| {
        serde_json::json!({
            "id": row.get::<String, _>("id"),
            "task_id": row.get::<String, _>("task_id"),
            "agent_id": row.get::<Option<String>, _>("agent_id"),
            "agent_name": row.get::<Option<String>, _>("agent_name"),
            "step": row.get::<i32, _>("step"),
            "action": row.get::<String, _>("action"),
            "latency_ms": row.get::<Option<i32>, _>("latency_ms"),
            "success": row.get::<bool, _>("success"),
            "timestamp": row.get::<String, _>("timestamp"),
        })
    }).collect::<Vec<_>>();

    Json(executions).into_response()
}

// Get single execution
pub async fn get_execution_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let row = match sqlx::query(
        "SELECT e.*, a.name as agent_name FROM executions e LEFT JOIN agents a ON e.agent_id = a.id WHERE e.id = ?"
    )
    .bind(&id)
    .fetch_optional(&state.db_pool)
    .await {
        Ok(r) => r,
        Err(e) => {
            warn!("Failed to fetch execution {}: {}", id, e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response();
        }
    };

    if let Some(row) = row {
        let execution = serde_json::json!({
            "id": row.get::<String, _>("id"),
            "task_id": row.get::<String, _>("task_id"),
            "agent_id": row.get::<Option<String>, _>("agent_id"),
            "agent_name": row.get::<Option<String>, _>("agent_name"),
            "step": row.get::<i32, _>("step"),
            "action": row.get::<String, _>("action"),
            "input_snapshot": row.get::<Option<serde_json::Value>, _>("input_snapshot"),
            "output_snapshot": row.get::<Option<serde_json::Value>, _>("output_snapshot"),
            "latency_ms": row.get::<Option<i32>, _>("latency_ms"),
            "success": row.get::<bool, _>("success"),
            "timestamp": row.get::<String, _>("timestamp"),
        });
        Json(execution).into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Execution not found" }))).into_response()
    }
}

// Get execution stats
pub async fn get_execution_stats_handler(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Get stats per agent
    let rows = match sqlx::query(
        r#"
        SELECT a.id, a.name, COUNT(e.id) as exec_count,
               SUM(CASE WHEN e.success = 1 THEN 1 ELSE 0 END) as success_count,
               AVG(e.latency_ms) as avg_latency
        FROM agents a
        LEFT JOIN executions e ON a.id = e.agent_id
        GROUP BY a.id
        ORDER BY exec_count DESC
        "#
    )
    .fetch_all(&state.db_pool)
    .await {
        Ok(rows) => rows,
        Err(e) => {
            warn!("Failed to fetch stats: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response();
        }
    };

    let stats = rows.iter().map(|row| {
        let exec_count: i64 = row.get("exec_count");
        let success_count: i64 = row.get("success_count");
        let success_rate = if exec_count > 0 { success_count as f64 / exec_count as f64 } else { 0.0 };

        serde_json::json!({
            "agent_id": row.get::<String, _>("id"),
            "agent_name": row.get::<String, _>("name"),
            "execution_count": exec_count,
            "success_rate": success_rate,
            "avg_latency_ms": row.get::<Option<f64>, _>("avg_latency"),
        })
    }).collect::<Vec<_>>();

    Json(stats).into_response()
}

// Agent-specific stats
pub async fn get_agent_stats_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.agent_service.get_all_agent_stats().await {
        Ok(all_stats) => {
            let stats = all_stats.iter()
                .find(|s| s["agent"]["id"] == id)
                .cloned();
            if let Some(stats) = stats {
                Json(stats).into_response()
            } else {
                (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Agent stats not found" }))).into_response()
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response(),
    }
}

// Agent endpoint to receive sub-tasks
pub async fn agent_execute_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<serde_json::Value>,
) -> impl IntoResponse {
    // This is a mock endpoint for agents to receive tasks
    // In a real implementation, this would be called by the task_service
    // Not intended for direct user calls
    (StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ "error": "Agents should not call this directly" }))).into_response()
}

#[derive(Debug, Deserialize)]
pub struct AgentExecutionRequest {
    pub task_id: String,
    pub sub_task_id: String,
    pub capability: String,
    pub input: serde_json::Value,
}