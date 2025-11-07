use dashmap::DashMap;
use log::info;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;
use wabble_core::message::server::ServerMessage;

#[derive(Default)]
pub struct ConnectionRegistry {
    connections: DashMap<Uuid, Sender<ServerMessage>>,
    user_connections: DashMap<Uuid, HashSet<Uuid>>,
    connection_user: DashMap<Uuid, Uuid>,
}

impl ConnectionRegistry {
    pub fn initialize() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn register_connection(&self, connection_id: Uuid, sender: Sender<ServerMessage>) {
        self.connections.insert(connection_id, sender);
        info!("Registered connection: '{connection_id}'");
    }

    pub fn unregister_connection(&self, connection_id: Uuid) {
        self.connections.remove(&connection_id);
        if let Some((_, user_id)) = self.connection_user.remove(&connection_id) {
            self.user_connections
                .entry(user_id)
                .and_modify(|connections| {
                    connections.retain(|id| *id != connection_id);
                });

            if let Some(entry) = self.user_connections.get(&user_id)
                && entry.value().is_empty()
            {
                drop(entry);
                self.user_connections.remove(&user_id);
            }
        }
        info!("Unregistered connection: '{connection_id}'");
    }

    pub fn register_user(&self, connection_id: Uuid, user_id: Uuid) {
        if !self.connections.contains_key(&connection_id) {
            return;
        }

        self.connection_user.insert(connection_id, user_id);
        self.user_connections
            .entry(user_id)
            .or_default()
            .insert(connection_id);

        info!("Registered connection '{connection_id}' for user '{user_id}'")
    }

    pub fn get_connection_user(&self, connection_id: Uuid) -> Option<Uuid> {
        self.connection_user
            .get(&connection_id)
            .map(|entry| entry.value().clone())
    }

    pub async fn send_to_connection(&self, connection_id: Uuid, message: ServerMessage) {
        if let Some(sender) = self.connections.get(&connection_id)
            && let Err(err) = sender.value().send(message).await
        {
            log::error!("[{connection_id}] Failed to send message: {err}");
        }
    }

    pub async fn send_to_user(&self, user_id: Uuid, message: ServerMessage) {
        if let Some(entry) = self.user_connections.get(&user_id) {
            for connection_id in entry.value() {
                self.send_to_connection(*connection_id, message.clone())
                    .await;
            }
        }
    }

    pub fn has_connection_user(&self, connection_id: Uuid, user_id: Uuid) -> bool {
        if let Some(users) = self.user_connections.get(&user_id) {
            users.value().contains(&connection_id)
        } else {
            false
        }
    }
}
