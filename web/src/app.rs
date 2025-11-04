use crate::ws::WebsocketClient;
use eframe::{Frame, Storage};
use egui::Context;
use log::{error, info};
use wabble_core::message::client::ClientMessage;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct WabbleApp {
    #[serde(skip, default)]
    ws: WebsocketClient,
}

impl WabbleApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app: WabbleApp = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        app.ws.connect("ws://127.0.0.1:8081").unwrap();
        app
    }
}

impl eframe::App for WabbleApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        for message in self.ws.receive().unwrap_or_default() {
            info!("Received message: {:?}", message);
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Wabble");

                ui.separator();

                if ui.button("Hello").clicked() {
                    let _ = self.ws.send(ClientMessage::Hello);
                }

                ui.separator();

                if ui.button("Connect").clicked() {
                    let _ = self.ws.connect("ws://127.0.0.1:8081");
                }

                if ui.button("Disconnect").clicked() {
                    self.ws.disconnect();
                }

                ui.separator();

                if self.ws.is_connected() {
                    ui.label("Connected");
                } else {
                    ui.label("Disconnected");
                }
            });
        });
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
