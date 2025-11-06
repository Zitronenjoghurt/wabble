use crate::state::ServerState;
use crate::websocket::connection::WebsocketConnection;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use log::error;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;
use wabble_core::message::client::ClientMessage;
use wabble_core::message::server::ServerMessage;

pub mod connection;
pub mod connections;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<ServerState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<ServerState>) {
    let id = Uuid::new_v4();

    let (ws_send, ws_receive) = socket.split();
    let (tx, rx) = tokio::sync::mpsc::channel::<ServerMessage>(100);

    let state_clone = state.clone();
    let send_task = tokio::spawn(async move { handle_send(id, ws_send, rx).await });
    let recv_task = tokio::spawn(async move {
        WebsocketConnection::new(id, state_clone)
            .handle_receive(ws_receive)
            .await
    });

    state.connections.register_connection(id, tx);

    tokio::select! {
        _ = send_task => {}
        _ = recv_task => {}
    }

    state.connections.unregister_connection(id);
}

async fn handle_send(
    id: Uuid,
    mut ws_send: SplitSink<WebSocket, Message>,
    mut rx: Receiver<ServerMessage>,
) {
    while let Some(message) = rx.recv().await {
        let encoded = match bincode::encode_to_vec(&message, bincode::config::standard()) {
            Ok(encoded) => encoded,
            Err(err) => {
                error!("[{id}] Failed to encode message for '{id}': {err}");
                continue;
            }
        };

        match ws_send.send(Message::binary(encoded)).await {
            Ok(_) => {}
            Err(err) => {
                error!("[{id}] Failed to send message to: {err}");
                break;
            }
        };
    }
}
