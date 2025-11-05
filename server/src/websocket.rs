use crate::state::ServerState;
use axum::extract::ws::WebSocket;
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures_util::{SinkExt, StreamExt};
use log::info;
use std::sync::Arc;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<ServerState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<ServerState>) {
    info!("New WebSocket connection");

    let (mut sender, mut receiver) = socket.split();
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            if sender.send(msg).await.is_err() {
                break;
            }
        } else {
            break;
        }
    }

    info!("WebSocket connection closed");
}
