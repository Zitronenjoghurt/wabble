use crate::systems::toasts::ToastSystem;
use crate::systems::ws::WebsocketClient;
use crate::types::timeout::Timeout;
use egui::{Button, TextBuffer, TextEdit, Widget};
use wabble_core::crypto::secret::Secret;
use wabble_core::message::client::ClientMessage;
use wabble_core::validate::{validate_invite_code, validate_password, validate_username};

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
    #[serde(skip, default)]
    remember_me: bool,
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
            remember_me: false,
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

    fn clear_input(&mut self) {
        self.state.username.clear();
        self.state.password.clear();
        self.state.confirm_password.clear();
        self.state.invite_code.clear();
    }

    fn validate_input(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if let Err(err) = validate_username(&self.state.username) {
            errors.push(err.to_string());
        };

        if let Err(err) = validate_password(&self.state.password) {
            errors.push(err.to_string());
        };

        if self.state.is_register {
            if let Err(err) = validate_invite_code(&self.state.invite_code) {
                errors.push(err.to_string());
            };

            if self.state.password != self.state.confirm_password {
                errors.push("Passwords do not match".to_string());
            };
        };

        errors
    }

    fn show_login(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            TextEdit::singleline(&mut self.state.username)
                .hint_text("Username")
                .char_limit(20)
                .show(ui);

            ui.add_space(5.0);

            TextEdit::singleline(&mut self.state.password)
                .hint_text("Password")
                .password(true)
                .char_limit(128)
                .show(ui);
        });
    }

    fn show_register(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            TextEdit::singleline(&mut self.state.username)
                .hint_text("Username")
                .char_limit(20)
                .show(ui);

            ui.add_space(5.0);

            TextEdit::singleline(&mut self.state.invite_code)
                .hint_text("Invite code")
                .char_limit(36)
                .show(ui);

            ui.add_space(5.0);

            TextEdit::singleline(&mut self.state.password)
                .hint_text("Password")
                .password(true)
                .char_limit(128)
                .show(ui);

            ui.add_space(5.0);

            TextEdit::singleline(&mut self.state.confirm_password)
                .hint_text("Confirm password")
                .password(true)
                .char_limit(128)
                .show(ui);
        });
    }

    fn handle_submit(&mut self) {
        self.state.submit_timeout.reset();

        let password = Secret::new(self.state.password.take());
        self.state.confirm_password.clear();
        let result = if self.state.is_register {
            self.ws.send(ClientMessage::Register {
                username: self.state.username.clone(),
                password,
                invite_code: self.state.invite_code.clone(),
            })
        } else {
            self.ws.send(ClientMessage::Login {
                username: self.state.username.clone(),
                password,
            })
        };

        match result {
            Ok(_) => {
                self.clear_input();
                if self.state.remember_me {
                    let _ = self.ws.send(ClientMessage::RequestSessionToken);
                }
            }
            Err(err) => self.toasts.error(err.to_string()),
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

            ui.checkbox(&mut self.state.remember_me, "Remember me");

            ui.separator();

            let validation_errors = self.validate_input();
            ui.horizontal(|ui| {
                if self.state.submit_timeout.is_ongoing() {
                    ui.spinner();
                };

                let button_response = ui.add_enabled(
                    self.ws.is_connected() && self.state.submit_timeout.is_expired() && validation_errors.is_empty(),
                    Button::new("Submit"),
                );

                if button_response.clicked() {
                    self.handle_submit();
                }
            });

            if !self.ws.is_connected() {
                ui.small("Server connection required, configurable via the connection button in the top bar");
            } else if !validation_errors.is_empty() {
                ui.small("Invalid input:");
                for error in validation_errors {
                    ui.small(error);
                }
            }
        })
        .response
    }
}
