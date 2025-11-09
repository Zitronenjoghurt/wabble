use crate::systems::ws::remember_me::RememberMe;
use crate::types::server_url::ServerUrl;
use anyhow::Context;
use ewebsock::{WsEvent, WsMessage, WsReceiver, WsSender};
use serde::{Deserialize, Serialize};
use wabble_core::crypto::secret::Secret;
use wabble_core::message::client::ClientMessage;
use wabble_core::message::server::{ServerAdminMessage, ServerError, ServerMessage};
use web_time::{Duration, Instant};

mod auth_state;
mod remember_me;
mod store;

#[derive(Default, Serialize, Deserialize)]
pub struct WebsocketClient {
    #[serde(skip, default)]
    sender: Option<WsSender>,
    #[serde(skip, default)]
    receiver: Option<WsReceiver>,
    #[serde(skip, default)]
    url: Option<ServerUrl>,
    #[serde(skip, default)]
    is_connected: bool,
    #[serde(skip, default)]
    just_connected: bool,
    #[serde(skip, default)]
    last_ping: Option<Instant>,
    #[serde(skip, default)]
    ping_timer: Option<Instant>,
    #[serde(skip, default)]
    ping: Option<Duration>,
    #[serde(skip, default)]
    auth_state: auth_state::AuthState,
    #[serde(skip, default)]
    store: store::WsStore,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    remember_me: Option<RememberMe>,
}

impl WebsocketClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    pub fn is_connecting(&self) -> bool {
        (self.sender.is_some() || self.receiver.is_some()) && !self.is_connected
    }

    pub fn url(&self) -> Option<&ServerUrl> {
        self.url.as_ref()
    }

    pub fn ping(&self) -> Option<Duration> {
        self.ping
    }

    pub fn auth_state(&self) -> &auth_state::AuthState {
        &self.auth_state
    }

    pub fn store(&self) -> &store::WsStore {
        &self.store
    }

    pub fn store_mut(&mut self) -> &mut store::WsStore {
        &mut self.store
    }

    pub fn remember_me(&self) -> Option<&RememberMe> {
        self.remember_me.as_ref()
    }

    pub fn connect(&mut self, url: &ServerUrl) -> anyhow::Result<()> {
        if self.is_connected {
            return Ok(());
        };

        let (sender, receiver) =
            ewebsock::connect(url.as_platform_specific_url(), Default::default())
                .map_err(anyhow::Error::msg)
                .context("Failed to connect")?;

        self.sender = Some(sender);
        self.receiver = Some(receiver);
        self.url = Some(url.clone());
        Ok(())
    }

    pub fn connect_with_remember_me(&mut self) -> anyhow::Result<()> {
        if let Some(remember_me) = self.remember_me.as_ref() {
            self.connect(&remember_me.url.clone())
        } else {
            Ok(())
        }
    }

    pub fn logout(&mut self) {
        self.disconnect();
        self.remember_me = None;
    }

    pub fn disconnect(&mut self) {
        self.clear_connection();
    }

    fn clear_connection(&mut self) {
        self.sender = None;
        self.receiver = None;
        self.url = None;
        self.is_connected = false;
        self.last_ping = None;
        self.ping_timer = None;
        self.ping = None;
        self.auth_state.clear();
        self.store.clear();
    }

    pub fn send(&mut self, message: ClientMessage) -> WebsocketResult<()> {
        if !self.is_connected {
            return Err(WebsocketError::NotConnected);
        }

        let Some(sender) = &mut self.sender else {
            return Err(WebsocketError::NotConnected);
        };

        let data = bincode::encode_to_vec(&message, bincode::config::standard())?;
        sender.send(WsMessage::Binary(data));

        Ok(())
    }

    pub fn update(&mut self) -> WebsocketResult<Vec<ServerMessage>> {
        self.update_store()?;
        self.handle_ping_timing()?;
        self.handle_just_connected()?;
        self.process_websocket_events()
    }

    fn update_store(&mut self) -> WebsocketResult<()> {
        if !self.auth_state.is_authenticated() {
            return Ok(());
        }

        if self.store.timer_friendship.is_expired() {
            self.store.timer_friendship.reset();
            self.update_friends();
            self.update_friend_requests();
        }

        Ok(())
    }

    pub fn update_friend_requests(&mut self) {
        let _ = self.send(ClientMessage::RetrieveFriendRequests);
    }

    pub fn update_friends(&mut self) {
        let _ = self.send(ClientMessage::RetrieveFriends);
    }

    fn handle_ping_timing(&mut self) -> WebsocketResult<()> {
        if let Some(last_ping) = self.last_ping {
            if last_ping.elapsed().as_secs() > 10 {
                self.ping_timer = Some(Instant::now());
                self.last_ping = Some(Instant::now());
                self.send(ClientMessage::Ping)?;
            }
        } else {
            self.last_ping = Some(Instant::now());
        }
        Ok(())
    }

    fn handle_just_connected(&mut self) -> WebsocketResult<()> {
        if self.just_connected {
            self.just_connected = false;
            if let Some(remember_me) = &self.remember_me {
                let id = remember_me.id.to_string();
                let token = Secret::new(remember_me.token.to_string());
                self.send(ClientMessage::LoginSession { id, token })?;
            }
        }
        Ok(())
    }

    fn process_websocket_events(&mut self) -> WebsocketResult<Vec<ServerMessage>> {
        let Some(receiver) = &mut self.receiver else {
            return Err(WebsocketError::NotConnected);
        };

        let mut messages = Vec::new();

        let events: Vec<WsEvent> = std::iter::from_fn(|| receiver.try_recv()).collect();
        for event in events {
            match event {
                WsEvent::Opened => {
                    self.handle_connection_opened();
                }
                WsEvent::Message(WsMessage::Binary(data)) => {
                    let message = self.decode_message(data)?;
                    self.handle_server_message(&message);
                    messages.push(message);
                }
                WsEvent::Error(err) => {
                    self.clear_connection();
                    return Err(WebsocketError::Error(err));
                }
                WsEvent::Closed => {
                    self.clear_connection();
                    return Err(WebsocketError::ConnectionClosed);
                }
                _ => {}
            }
        }

        Ok(messages)
    }

    fn handle_connection_opened(&mut self) {
        self.is_connected = true;
        self.just_connected = true;
    }

    fn decode_message(&self, data: Vec<u8>) -> WebsocketResult<ServerMessage> {
        let (message, _) = bincode::decode_from_slice(&data, bincode::config::standard())?;
        Ok(message)
    }

    fn handle_server_message(&mut self, message: &ServerMessage) {
        match message {
            ServerMessage::Pong => {
                if let Some(ping_timer) = self.ping_timer.take() {
                    self.ping = Some(ping_timer.elapsed());
                }
            }
            ServerMessage::Authenticated(me) => {
                self.auth_state.set_authenticated(me.clone());
            }
            ServerMessage::Error(ServerError::SessionInvalid) => {
                self.handle_session_invalid();
            }
            ServerMessage::Error(ServerError::Unauthorized) => {
                self.clear_connection();
            }
            ServerMessage::Admin(ServerAdminMessage::InviteCodes(codes)) => {
                self.store.invite_codes = codes.clone();
            }
            ServerMessage::SessionToken { id, token } => {
                self.handle_session_token(id, token);
            }
            ServerMessage::FriendRequests(requests) => {
                self.store.friend_requests = requests.clone();
            }
            ServerMessage::Friends(requests) => {
                self.store.friends = requests.clone();
            }
            _ => {}
        }
    }

    fn handle_session_invalid(&mut self) {
        self.auth_state.clear();
        self.remember_me = None;
    }

    fn handle_session_token(&mut self, id: &str, token: &Secret) {
        if let Some(server_url) = self.url.as_ref() {
            self.remember_me = Some(RememberMe {
                url: server_url.clone(),
                id: id.to_string(),
                token: token.reveal_str().to_string(),
            });
        }
    }
}

pub type WebsocketResult<T> = Result<T, WebsocketError>;

#[derive(Debug, thiserror::Error)]
pub enum WebsocketError {
    #[error("Connection closed")]
    ConnectionClosed,
    #[error("Failed to decode message: {0}")]
    Decode(#[from] bincode::error::DecodeError),
    #[error("Failed to encode message: {0}")]
    Encode(#[from] bincode::error::EncodeError),
    #[error("Websocket error: {0}")]
    Error(String),
    #[error("Not connected")]
    NotConnected,
}
