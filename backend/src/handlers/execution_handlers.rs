use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use crate::db::Repository;

#[derive(Deserialize)]
pub struct ExecutionsQuery {
    pub task_id: Option<String>,
    pub limit: Option<i64>,
}

pub async fn get_executions(
    State(repo): State<Repository>,
    Query(query): Query<ExecutionsQuery>,
) -> (StatusCode, Json<Vec<Execution>>) {
    if let Some(task_id) = query.task_id {
        match repo.get_executions_for_task(&task_id, query.limit).await {
            Ok(executions) => (StatusCode::OK, Json(executions)),
            Err(e) => {
                eprintln!("Error fetching executions for task {}: {}", task_id, e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new()))
            }
        }
    } else {
        // For now, require task_id. In production, could add filtering without it
        (
            StatusCode::BAD_REQUEST,
            Json(Vec::new()),
        )
    }
}

pub async fn get_execution_stats(
    State(repo): State<Repository>,
) -> (StatusCode, Json<ExecutionStats>) {
    match repo.get_execution_stats().await {
        Ok(stats) => (StatusCode::OK, Json(stats)),
        Err(e) => {
            eprintln!("Error fetching execution stats: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ExecutionStats {
                    total_executions: 0,
                    success_rate: 0.0,
                    avg_latency_ms: 0.0,
                    by_agent: Vec::new(),
                }),
            )
        }
    }
}
