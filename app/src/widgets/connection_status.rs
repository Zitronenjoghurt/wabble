use crate::systems::ws::WebsocketClient;
use crate::types::server_url::ServerUrl;
use egui::{Button, Grid, Response, TextEdit, Ui, Widget};

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

        let server_url = ServerUrl::parse(self.url_buf);

        ui.horizontal(|ui| {
            let connect_button = ui.add_enabled(server_url.is_some(), Button::new("Connect"));
            if connect_button.clicked()
                && let Some(server_url) = &server_url
            {
                let _ = self.ws.connect(server_url);
            }

            if let Some(remember_me) = self.ws.remember_me()
                && ui
                    .button(format!(
                        "Login at {}",
                        remember_me.url.as_human_readable_url()
                    ))
                    .clicked()
            {
                let _ = self.ws.connect_with_remember_me();
            }
        });

        if server_url.is_none() {
            ui.small("Invalid server URL");
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
                ui.label(
                    self.ws
                        .url()
                        .map(|url| url.as_human_readable_url())
                        .unwrap_or("unknown".to_string()),
                );
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
