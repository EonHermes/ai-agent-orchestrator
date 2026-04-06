use axum::{
    extract::{WebSocket, State},
    response::Response,
};
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use serde_json;
use crate::{error::Result, models::TaskBroadcast, ws::broadcast::BroadcastState};

mod broadcast {
    use tokio::sync::broadcast;

    #[derive(Clone)]
    pub struct BroadcastState {
        tx: broadcast::Sender<Vec<u8>>,
    }

    impl BroadcastState {
        pub fn new() -> Self {
            let (tx, _) = broadcast::channel(100);
            Self { tx }
        }

        pub fn subscribe(&self) -> broadcast::Receiver<Vec<u8>> {
            self.tx.subscribe()
        }

        pub async fn broadcast(&self, message: Vec<u8>) -> Result<(), broadcast::error::SendError<Vec<u8>>> {
            self.tx.send(message).map(|_| ())
        }
    }
}

use broadcast::BroadcastState;

#[derive(Clone)]
pub struct WebSocketClients {
    pub broadcast: BroadcastState,
}

impl WebSocketClients {
    pub fn new() -> Self {
        Self {
            broadcast: BroadcastState::new(),
        }
    }

    pub async fn broadcast_task_created(&self, task: crate::models::Task) -> Result<()> {
        let msg = TaskBroadcast::created(task);
        let data = serde_json::to_vec(&msg)
            .map_err(|e| crate::error::Error::Ws(e.to_string()))?;
        self.broadcast.broadcast(data).await
            .map_err(|e| crate::error::Error::Ws(e.to_string()))?;
        Ok(())
    }

    pub async fn broadcast_task_updated(&self, task: crate::models::Task) -> Result<()> {
        let msg = TaskBroadcast::updated(task);
        let data = serde_json::to_vec(&msg)
            .map_err(|e| crate::error::Error::Ws(e.to_string()))?;
        self.broadcast.broadcast(data).await
            .map_err(|e| crate::error::Error::Ws(e.to_string()))?;
        Ok(())
    }

    pub async fn broadcast_task_deleted(&self, task_id: uuid::Uuid) -> Result<()> {
        let msg = TaskBroadcast::deleted(task_id);
        let data = serde_json::to_vec(&msg)
            .map_err(|e| crate::error::Error::Ws(e.to_string()))?;
        self.broadcast.broadcast(data).await
            .map_err(|e| crate::error::Error::Ws(e.to_string()))?;
        Ok(())
    }
}

pub async fn handle_websocket(
    State(state): State<Arc<crate::api::AppState>>,
    ws: WebSocket,
) -> Result<Response> {
    let (sender, mut receiver) = ws.split();
    let mut broadcast_rx = state.ws_clients.broadcast.subscribe();

    // Spawn task to forward broadcast messages to this client
    let sender_task = tokio::spawn(async move {
        loop {
            match broadcast_rx.recv().await {
                Ok(msg) => {
                    if sender.send(axum::extract::ws::Message::Binary(msg)).await.is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    // Handle incoming messages from client
    let mut receive_task = tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(axum::extract::ws::Message::Text(text)) => {
                    // Handle client messages if needed (ping/pong, subscriptions)
                    tracing::debug!("Received WS message: {}", text);
                }
                Ok(axum::extract::ws::Message::Close(_)) => {
                    break;
                }
                Err(e) => {
                    tracing::error!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = sender_task => {
            receive_task.abort();
        }
        _ = receive_task => {
            sender_task.abort();
        }
    }

    Ok(Response::new(()))
}
