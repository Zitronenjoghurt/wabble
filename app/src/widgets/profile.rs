use crate::systems::ws::WebsocketClient;
use egui::Widget;

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
            ui.label(&me.username);
            if ui.button("Logout").clicked() {
                self.ws.logout();
            }
        })
        .response
    }
}
