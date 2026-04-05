use axum::{
    extract::{Path, Query, State, Extension},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{
    db::Repository,
    models::*,
    services::executor::Executor,
};

#[derive(Deserialize)]
pub struct ListTasksQuery {
    pub status: Option<String>,
    pub agent_id: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn create_task(
    State(repo): State<Repository>,
    Json(new_task): Json<NewTask>,
) -> (StatusCode, Json<TaskResponse>) {
    match repo.create_task(&new_task).await {
        Ok(task) => (
            StatusCode::CREATED,
            Json(TaskResponse {
                task,
                sub_tasks: Vec::new(),
            }),
        ),
        Err(e) => {
            eprintln!("Error creating task: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: UserTask::new("".to_string()),
                    sub_tasks: Vec::new(),
                }),
            )
        }
    }
}

pub async fn list_tasks(
    State(repo): State<Repository>,
    Query(query): Query<ListTasksQuery>,
) -> (StatusCode, Json<Vec<UserTask>>) {
    match repo
        .list_tasks(query.status.as_deref(), query.agent_id.as_deref(), query.limit, query.offset)
        .await
    {
        Ok(tasks) => (StatusCode::OK, Json(tasks)),
        Err(e) => {
            eprintln!("Error listing tasks: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new()))
        }
    }
}

pub async fn get_task(
    State(repo): State<Repository>,
    Path(id): Path<String>,
) -> (StatusCode, Json<TaskResponse>) {
    match repo.get_task(&id).await {
        Ok(Some(task)) => {
            match repo.get_sub_tasks_for_task(&id).await {
                Ok(sub_tasks) => (
                    StatusCode::OK,
                    Json(TaskResponse { task, sub_tasks }),
                ),
                Err(e) => {
                    eprintln!("Error fetching sub-tasks for task {}: {}", id, e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(TaskResponse { task, sub_tasks: Vec::new() }),
                    )
                }
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(TaskResponse {
            task: UserTask::new("".to_string()),
            sub_tasks: Vec::new(),
        })),
        Err(e) => {
            eprintln!("Error fetching task {}: {}", id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: UserTask::new("".to_string()),
                    sub_tasks: Vec::new(),
                }),
            )
        }
    }
}

pub async fn cancel_task(
    State(repo): State<Repository>,
    Path(id): Path<String>,
) -> StatusCode {
    match repo.update_task_status(id, TaskStatus::Cancelled, None).await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            eprintln!("Error cancelling task {}: {}", id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn submit_task(
    State(repo): State<Repository>,
    Extension(executor): Extension<Arc<Executor>>,
    Json(task_submission): Json<TaskSubmission>,
) -> (StatusCode, Json<TaskResponse>) {
    // Create the task
    let new_task = NewTask {
        user_query: task_submission.user_query,
    };

    let task = match repo.create_task(&new_task).await {
        Ok(task) => task,
        Err(e) => {
            eprintln!("Error creating task: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: UserTask::new("".to_string()),
                    sub_tasks: Vec::new(),
                }),
            );
        }
    };

    // Spawn execution in background
    let task_id = task.id.clone();
    let executor = executor.clone();
    let repo_for_task = Arc::new(repo.clone());

    tokio::spawn(async move {
        if let Err(e) = executor.execute_task(&task_id).await {
            error!("Task {} execution failed: {}", task_id, e);
            let _ = repo_for_task
                .update_task_status(&task_id, TaskStatus::Failed, Some(&e.to_string()))
                .await;
        }
    });

    // Return immediately with task info
    (
        StatusCode::ACCEPTED,
        Json(TaskResponse {
            task,
            sub_tasks: Vec::new(), // Will be filled by GET /tasks/:id
        }),
    )
}
