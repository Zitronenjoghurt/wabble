use crate::systems::ws::WebsocketClient;
use egui::{Grid, Response, TextEdit, Ui, Widget};

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

        if let Some(remember_me) = self.ws.remember_me()
            && ui.button(format!("Login at {}", remember_me.url)).clicked()
        {
            let _ = self.ws.connect_with_remember_me();
        }
    }

    fn show_connecting(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.spinner();
            ui.label("Connecting to server...");
        });
    }

    fn show_connected(&mut self, ui: &mut Ui) {
        ui.label("Connected");

        ui.separator();

        Grid::new("connection_status_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Server");
                ui.label(self.ws.url().unwrap_or("unknown").to_string());
                ui.end_row();

                ui.label("Ping");
                ui.label(format!("{:?}", self.ws.ping().unwrap_or_default()));
                ui.end_row();
            });

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
