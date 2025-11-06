use crate::systems::ws::WebsocketClient;
use egui::{Response, TextEdit, Ui, Widget};

pub struct ConnectionStatus<'a> {
    ws: &'a mut WebsocketClient,
    url_buf: &'a mut String,
}

impl<'a> ConnectionStatus<'a> {
    pub fn new(ws: &'a mut WebsocketClient, url_buf: &'a mut String) -> Self {
        Self { ws, url_buf }
    }

    fn show_disconnected(&mut self, ui: &mut Ui) {
        ui.label("Disconnected");

        ui.separator();

        TextEdit::singleline(self.url_buf)
            .hint_text("Server URL")
            .ui(ui);

        if ui.button("Connect").clicked() {
            let _ = self.ws.connect(&format!("{}/ws", self.url_buf));
        }
    }

    fn show_connecting(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.spinner();
            ui.label("Connecting to server...");
        });
    }

    fn show_connected(&mut self, ui: &mut Ui) {
        ui.label(format!(
            "Connected to {}",
            self.ws.url().unwrap_or("unknown")
        ));

        ui.label(format!("Ping: {:?}", self.ws.ping()));

        ui.separator();

        if ui.button("Disconnect").clicked() {
            self.ws.disconnect();
        }
    }
}

impl Widget for ConnectionStatus<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            if !self.ws.is_connecting() && !self.ws.is_connected() {
                self.show_disconnected(ui)
            } else if self.ws.is_connecting() {
                self.show_connecting(ui)
            } else {
                self.show_connected(ui)
            }
        })
        .response
    }
}
