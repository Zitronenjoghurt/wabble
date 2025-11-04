use anyhow::Context;
use ewebsock::{WsEvent, WsMessage, WsReceiver, WsSender};
use wabble_core::message::client::ClientMessage;
use wabble_core::message::server::ServerMessage;

#[derive(Default)]
pub struct WebsocketClient {
    sender: Option<WsSender>,
    receiver: Option<WsReceiver>,
    is_connected: bool,
}

impl WebsocketClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected
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
        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.clear_connection();
    }

    fn clear_connection(&mut self) {
        self.sender = None;
        self.receiver = None;
        self.is_connected = false;
    }

    pub fn send(&mut self, message: ClientMessage) -> anyhow::Result<()> {
        if !self.is_connected {
            return Err(anyhow::anyhow!("Not connected"));
        }

        let Some(sender) = &mut self.sender else {
            return Err(anyhow::anyhow!("Not connected"));
        };

        let data = bincode::encode_to_vec(&message, bincode::config::standard())
            .context("Failed to encode message")?;
        sender.send(WsMessage::Binary(data));

        Ok(())
    }

    pub fn receive(&mut self) -> anyhow::Result<Vec<ServerMessage>> {
        let Some(receiver) = &mut self.receiver else {
            return Err(anyhow::anyhow!("Not connected"));
        };

        let mut messages = Vec::new();
        while let Some(event) = receiver.try_recv() {
            match event {
                WsEvent::Opened => {
                    self.is_connected = true;
                }
                WsEvent::Message(WsMessage::Binary(data)) => {
                    let (message, _) =
                        bincode::decode_from_slice(&data, bincode::config::standard())
                            .context("Failed to decode message")?;
                    messages.push(message);
                }
                WsEvent::Error(err) => {
                    self.clear_connection();
                    return Err(anyhow::anyhow!("Websocket error: {}", err));
                }
                WsEvent::Closed => {
                    self.clear_connection();
                    return Err(anyhow::anyhow!("Websocket closed"));
                }
                _ => {}
            }
        }

        Ok(messages)
    }
}
