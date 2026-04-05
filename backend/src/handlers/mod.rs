pub mod agent_handlers;
pub mod task_handlers;
pub mod execution_handlers;
pub mod system_handlers;

use axum::{
    routing::{get, post, put, delete},
    Router,
};

use crate::{db::Repository, services::{executor::Executor, llm::LlmService}};

use self::{
    agent_handlers::{create_agent, delete_agent, get_agent, list_agents, update_agent},
    execution_handlers::{get_execution_stats, get_executions},
    system_handlers::{health_check, parse_only, status},
    task_handlers::{cancel_task, create_task, get_task, list_tasks, submit_task},
};

pub fn create_routes(
    repo: Repository,
    llm: Arc<LlmService>,
    executor: Arc<Executor>,
) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/status", get(status))
        .route("/api/v1/agents", post(create_agent))
        .route("/api/v1/agents", get(list_agents))
        .route("/api/v1/agents/:id", get(get_agent))
        .route("/api/v1/agents/:id", put(update_agent))
        .route("/api/v1/agents/:id", delete(delete_agent))
        .route("/api/v1/agents/capabilities", get(list_agents)) // Will reuse list to get capabilities
        .route("/api/v1/tasks", post(create_task))
        .route("/api/v1/tasks", get(list_tasks))
        .route("/api/v1/tasks/:id", get(get_task))
        .route("/api/v1/tasks/:id/cancel", post(cancel_task))
        .route("/api/v1/tasks/submit", post(submit_task))
        .route("/api/v1/executions", get(get_executions))
        .route("/api/v1/executions/stats", get(get_execution_stats))
        .route("/api/v1/parse", post(parse_only))
        .with_state(ServiceState { repo, executor })
}

#[derive(Clone)]
pub struct ServiceState {
    pub repo: Repository,
    pub executor: Arc<Executor>,
}
