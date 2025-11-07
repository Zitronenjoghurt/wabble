use anyhow::Context;
use ewebsock::{WsEvent, WsMessage, WsReceiver, WsSender};
use wabble_core::message::client::ClientMessage;
use wabble_core::message::server::{ServerAdminMessage, ServerError, ServerMessage};
use web_time::{Duration, Instant};

mod auth_state;
mod store;

#[derive(Default)]
pub struct WebsocketClient {
    sender: Option<WsSender>,
    receiver: Option<WsReceiver>,
    url: Option<String>,
    is_connected: bool,
    last_ping: Option<Instant>,
    ping_timer: Option<Instant>,
    ping: Option<Duration>,
    auth_state: auth_state::AuthState,
    store: store::WsStore,
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

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
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

    pub fn connect(&mut self, url: &str) -> anyhow::Result<()> {
        if self.is_connected {
            return Ok(());
        };

        let (sender, receiver) = ewebsock::connect(url, Default::default())
            .map_err(anyhow::Error::msg)
            .context("Failed to connect")?;

        self.sender = Some(sender);
        self.receiver = Some(receiver);
        self.url = Some(url.to_string());
        Ok(())
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

    pub fn receive(&mut self) -> WebsocketResult<Vec<ServerMessage>> {
        if let Some(last_ping) = self.last_ping {
            if last_ping.elapsed().as_secs() > 10 {
                self.ping_timer = Some(Instant::now());
                self.last_ping = Some(Instant::now());
                let _ = self.send(ClientMessage::Ping);
            }
        } else {
            self.last_ping = Some(Instant::now());
        }

        let Some(receiver) = &mut self.receiver else {
            return Err(WebsocketError::NotConnected);
        };

        let mut messages = Vec::new();
        while let Some(event) = receiver.try_recv() {
            match event {
                WsEvent::Opened => {
                    self.is_connected = true;
                }
                WsEvent::Message(WsMessage::Binary(data)) => {
                    let (message, _) =
                        bincode::decode_from_slice(&data, bincode::config::standard())?;

                    match &message {
                        ServerMessage::Pong => {
                            if let Some(ping_timer) = self.ping_timer.take() {
                                self.ping = Some(ping_timer.elapsed());
                            }
                        }
                        ServerMessage::LoginSuccess(permissions) => {
                            self.auth_state.set_authenticated(*permissions);
                        }
                        ServerMessage::AlreadyLoggedIn(permissions) => {
                            self.auth_state.set_authenticated(*permissions);
                        }
                        ServerMessage::Error(ServerError::Unauthorized) => {
                            self.clear_connection();
                            return Err(WebsocketError::NotConnected);
                        }
                        ServerMessage::Admin(ServerAdminMessage::InviteCodes(codes)) => {
                            self.store.invite_codes = codes.clone();
                        }
                        _ => {}
                    }

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
