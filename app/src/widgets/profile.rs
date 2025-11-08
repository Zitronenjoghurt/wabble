use crate::systems::ws::WebsocketClient;
use egui::{Grid, Widget};

pub struct ProfileWidget<'a> {
    ws: &'a mut WebsocketClient,
}

impl<'a> ProfileWidget<'a> {
    pub fn new(ws: &'a mut WebsocketClient) -> Self {
        Self { ws }
    }
}

impl Widget for ProfileWidget<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            let Some(me) = self.ws.auth_state().me() else {
                return;
            };

            Grid::new("profile_grid")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Username");
                    ui.label(&me.username);
                    ui.end_row();

                    ui.label("Friend code");
                    ui.label(&me.friend_code);
                    ui.end_row();
                });

            ui.separator();

            if ui.button("Logout").clicked() {
                self.ws.logout();
            }
        })
        .response
    }
}
