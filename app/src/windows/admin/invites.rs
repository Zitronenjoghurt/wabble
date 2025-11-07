use crate::widgets::simple_list::SimpleList;
use crate::WabbleApp;
use egui::{Ui, Widget};
use wabble_core::message::client::{ClientAdminCommand, ClientMessage};

pub fn render_invites_tab(app: &mut WabbleApp, ui: &mut Ui) {
    ui.horizontal(|ui| {
        if ui.button("Fetch").clicked() {
            let _ = app.ws.send(ClientMessage::Admin(
                ClientAdminCommand::RetrieveInviteCodes,
            ));
        }

        if ui.button("Generate 5").clicked() {
            let _ = app.ws.send(ClientMessage::Admin(
                ClientAdminCommand::GenerateInviteCodes(5),
            ));
        }
    });

    ui.separator();
    SimpleList::new(&app.ws.store().invite_codes, "invite_codes").ui(ui);
}
