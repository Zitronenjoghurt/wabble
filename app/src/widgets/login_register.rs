use crate::systems::ws::WebsocketClient;
use egui::{TextEdit, Widget};

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct LoginRegisterState {
    is_register: bool,
    username: String,
    #[serde(skip, default)]
    password: String,
    #[serde(skip, default)]
    confirm_password: String,
    #[serde(skip, default)]
    invite_code: String,
}

pub struct LoginRegister<'a> {
    state: &'a mut LoginRegisterState,
    ws: &'a mut WebsocketClient,
}

impl<'a> LoginRegister<'a> {
    pub fn new(state: &'a mut LoginRegisterState, ws: &'a mut WebsocketClient) -> Self {
        Self { state, ws }
    }

    fn show_login(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            TextEdit::singleline(&mut self.state.username)
                .hint_text("Username")
                .show(ui);

            ui.add_space(5.0);

            TextEdit::singleline(&mut self.state.password)
                .hint_text("Password")
                .password(true)
                .show(ui);
        });
    }

    fn show_register(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            TextEdit::singleline(&mut self.state.username)
                .hint_text("Username")
                .show(ui);

            ui.add_space(5.0);

            TextEdit::singleline(&mut self.state.invite_code)
                .hint_text("Invite code")
                .show(ui);

            ui.add_space(5.0);

            TextEdit::singleline(&mut self.state.password)
                .hint_text("Password")
                .password(true)
                .show(ui);

            ui.add_space(5.0);

            TextEdit::singleline(&mut self.state.confirm_password)
                .hint_text("Confirm password")
                .password(true)
                .show(ui);
        });
    }
}

impl Widget for LoginRegister<'_> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.state.is_register, false, "Login");
                ui.selectable_value(&mut self.state.is_register, true, "Register");
            });

            ui.separator();

            if self.state.is_register {
                self.show_register(ui);
            } else {
                self.show_login(ui);
            }

            ui.separator();

            if ui.button("Submit").clicked() {}
        })
        .response
    }
}
