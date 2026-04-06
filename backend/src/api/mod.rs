use axum::{
    routing::{get, post, put, delete},
    extract::{State, WebSocket, Path, Query, Json},
    response::{Response, IntoResponse},
    http::StatusCode,
};
use std::sync::Arc;
use sqlx::PgPool;
use crate::{db, error::Result, models::{self, Task, CreateTask, UpdateTask, TaskFilter, ApiResponse}};
use serde::Deserialize;
use tower_http::cors::CorsLayer;
use crate::ws::handle_websocket;

pub fn router(db_pool: PgPool, config: Arc<config::Config>) -> axum::Router {
    let cors = CorsLayer::new()
        .allow_origin(config.get_string("CORS_ALLOW_ORIGIN").unwrap_or("http://localhost:3000".to_string()))
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::PUT, axum::http::Method::DELETE])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let state = Arc::new(AppState { db_pool, ws_clients: Arc::new(ws::WebSocketClients::new()) });

    axum::Router::new()
        .route("/api/health", get(health))
        .route("/api/tasks", get(list_tasks).post(create_task))
        .route("/api/tasks/:id", get(get_task).put(update_task).delete(delete_task))
        .route("/ws", get(handle_websocket))
        .layer(cors)
        .with_state(state)
}

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub ws_clients: Arc<ws::WebSocketClients>,
}

async fn health(State(_state): State<Arc<AppState>>) -> Result<Json<ApiResponse<&'static str>>> {
    Ok(Json(ApiResponse::new("OK")))
}

async fn list_tasks(
    State(state): State<Arc<AppState>>,
    Query(filter): Query<TaskFilter>,
) -> Result<Json<ApiResponse<Vec<Task>>>> {
    let tasks = db::tasks::list(&state.db_pool, filter).await?;
    Ok(Json(ApiResponse::new(tasks)))
}

async fn get_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<ApiResponse<Task>>> {
    let task = db::tasks::get(&state.db_pool, id).await?
        .ok_or_else(|| crate::error::Error::NotFound(format!("Task {} not found", id)))?;
    Ok(Json(ApiResponse::new(task)))
}

async fn create_task(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<ApiResponse<Task>>> {
    let task = db::tasks::create(&state.db_pool, payload).await?;

    // Broadcast via WebSocket
    state.ws_clients.broadcast_task_created(task.clone()).await?;

    Ok(Json(ApiResponse::new(task)))
}

async fn update_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<ApiResponse<Task>>> {
    let task = db::tasks::update(&state.db_pool, id, payload).await?;

    // Broadcast via WebSocket
    state.ws_clients.broadcast_task_updated(task.clone()).await?;

    Ok(Json(ApiResponse::new(task)))
}

async fn delete_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Response> {
    db::tasks::delete(&state.db_pool, id).await?;

    // Broadcast via WebSocket
    state.ws_clients.broadcast_task_deleted(id).await?;

    Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(axum::body::Empty::new())
        .map_err(Into::into)
}
