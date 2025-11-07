use crate::systems::toasts::ToastSystem;
use crate::systems::ws::WebsocketClient;
use crate::types::timeout::Timeout;
use egui::{Button, TextEdit, Widget};
use wabble_core::message::client::ClientMessage;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LoginRegisterState {
    is_register: bool,
    username: String,
    #[serde(skip, default)]
    password: String,
    #[serde(skip, default)]
    confirm_password: String,
    #[serde(skip, default)]
    invite_code: String,
    #[serde(skip, default = "default_submit_timeout")]
    submit_timeout: Timeout,
}

impl Default for LoginRegisterState {
    fn default() -> Self {
        Self {
            is_register: Default::default(),
            username: Default::default(),
            password: Default::default(),
            confirm_password: Default::default(),
            invite_code: Default::default(),
            submit_timeout: default_submit_timeout(),
        }
    }
}

fn default_submit_timeout() -> Timeout {
    Timeout::new(web_time::Duration::from_secs(2))
}

pub struct LoginRegister<'a> {
    state: &'a mut LoginRegisterState,
    ws: &'a mut WebsocketClient,
    toasts: &'a mut ToastSystem,
}

impl<'a> LoginRegister<'a> {
    pub fn new(
        state: &'a mut LoginRegisterState,
        ws: &'a mut WebsocketClient,
        toasts: &'a mut ToastSystem,
    ) -> Self {
        Self { state, ws, toasts }
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

    fn handle_submit(&mut self) {
        self.state.submit_timeout.reset();
        if !self.state.is_register
            && let Err(err) = self.ws.send(ClientMessage::Login {
                username: self.state.username.clone(),
                password: self.state.password.clone(),
            })
        {
            self.toasts.error(err.to_string());
        }
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

            ui.horizontal(|ui| {
                if self.state.submit_timeout.is_ongoing() {
                    ui.spinner();
                };

                let button_response = ui.add_enabled(
                    self.ws.is_connected() && self.state.submit_timeout.is_expired(),
                    Button::new("Submit"),
                );

                if button_response.clicked() {
                    self.handle_submit();
                }
            });

            if !self.ws.is_connected() {
                ui.small("Server connection required, configurable via the connection button in the top bar");
            }
        })
        .response
    }
}
