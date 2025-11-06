use crate::crypto::secret::Secret;
use crate::crypto::verify_password;
use crate::state::ServerState;
use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitStream;
use futures_util::StreamExt;
use log::{error, info};
use std::sync::Arc;
use uuid::Uuid;
use wabble_core::message::client::ClientMessage;
use wabble_core::message::server::{ServerError, ServerMessage, ServerResult};

pub struct WebsocketConnection {
    id: Uuid,
    state: Arc<ServerState>,
}

impl WebsocketConnection {
    pub fn new(conn_id: Uuid, state: Arc<ServerState>) -> Self {
        Self { id: conn_id, state }
    }

    pub async fn handle_receive(&self, mut ws_receive: SplitStream<WebSocket>) {
        while let Some(Ok(message)) = ws_receive.next().await {
            match message {
                Message::Binary(data) => {
                    match bincode::decode_from_slice(data.as_ref(), bincode::config::standard()) {
                        Ok((client_message, _)) => {
                            self.handle_client_message(client_message).await;
                        }
                        Err(e) => {
                            error!("[{}] Failed to decode message: {e}", self.id);
                        }
                    }
                }
                Message::Close(_) => break,
                _ => {}
            };
        }
    }

    async fn send_to_connection(&self, message: ServerMessage) {
        self.state
            .connections
            .send_to_connection(self.id, message)
            .await;
    }

    async fn handle_client_message(&self, message: ClientMessage) {
        let result = match message {
            ClientMessage::Ping => {
                self.send_to_connection(ServerMessage::Pong).await;
                Ok(())
            }
            ClientMessage::Login { username, password } => {
                self.handle_login(username, Secret::new(password)).await
            }
            _ => Ok(()),
        };

        if let Err(err) = result {
            self.send_to_connection(ServerMessage::Error(err)).await;
        }
    }

    async fn handle_login(&self, username: String, password: Secret) -> ServerResult<()> {
        let user = self
            .state
            .stores
            .user
            .fetch_by_username(&username)
            .await
            .map_err(|_| ServerError::Database)?
            .ok_or(ServerError::InvalidCredentials)?;

        if !verify_password(&password, user.password_hash) {
            return Err(ServerError::InvalidCredentials);
        };
        drop(password);

        if self.state.connections.has_connection_user(self.id, user.id) {
            self.send_to_connection(ServerMessage::AlreadyLoggedIn)
                .await;
            return Ok(());
        };

        self.state.connections.register_user(self.id, user.id);
        self.send_to_connection(ServerMessage::LoginSuccess).await;
        info!("[{}] Logged in as '{}'", self.id, username);

        Ok(())
    }
}
