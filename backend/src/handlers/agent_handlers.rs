use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{db::Repository, models::*};

#[derive(Deserialize)]
pub struct ListAgentsQuery {
    pub status: Option<String>,
}

pub async fn list_agents(
    State(repo): State<Repository>,
    Query(query): Query<ListAgentsQuery>,
) -> (StatusCode, Json<AgentListResponse>) {
    match repo.list_agents(query.status.as_deref()).await {
        Ok(agents) => (
            StatusCode::OK,
            Json(AgentListResponse {
                agents,
                total: agents.len(),
            }),
        ),
        Err(e) => {
            eprintln!("Error listing agents: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AgentListResponse {
                    agents: Vec::new(),
                    total: 0,
                }),
            )
        }
    }
}

pub async fn get_agent(
    State(repo): State<Repository>,
    Path(id): Path<String>,
) -> (StatusCode, Json<Option<Agent>>) {
    match repo.get_agent(&id).await {
        Ok(agent) => (StatusCode::OK, Json(agent)),
        Err(e) => {
            eprintln!("Error fetching agent {}: {}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
        }
    }
}

pub async fn create_agent(
    State(repo): State<Repository>,
    Json(new_agent): Json<NewAgent>,
) -> (StatusCode, Json<Agent>) {
    match repo.create_agent(&new_agent).await {
        Ok(agent) => (StatusCode::CREATED, Json(agent)),
        Err(e) => {
            eprintln!("Error creating agent: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Agent::new(
                    "error".to_string(),
                    None,
                    "".to_string(),
                    Vec::new(),
                    None,
                )),
            )
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct UpdateAgentRequest {
    pub status: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

pub async fn update_agent(
    State(repo): State<Repository>,
    Path(id): Path<String>,
    Json(update): Json<UpdateAgentRequest>,
) -> (StatusCode, Json<Option<Agent>>) {
    if let Some(status_str) = &update.status {
        let status = match status_str.as_str() {
            "active" => AgentStatus::Active,
            "inactive" => AgentStatus::Inactive,
            "error" => AgentStatus::Error,
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(None),
                );
            }
        };
        if let Err(e) = repo.update_agent_status(&id, status).await {
            eprintln!("Error updating agent status: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
        }
    }

    // Note: For simplicity, full update (name, endpoint, etc.) would need another method
    // In production, you'd implement a full update function

    match repo.get_agent(&id).await {
        Ok(Some(agent)) => (StatusCode::OK, Json(Some(agent))),
        _ => (StatusCode::NOT_FOUND, Json(None)),
    }
}

pub async fn delete_agent(
    State(repo): State<Repository>,
    Path(id): Path<String>,
) -> StatusCode {
    match repo.delete_agent(&id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) => {
            eprintln!("Error deleting agent {}: {}", id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
