use crate::views::View;
use crate::widgets::login_register::{LoginRegister, LoginRegisterState};
use crate::windows::connection::ConnectionWindow;
use crate::windows::{AppWindow, ToggleableWindow};
use crate::WabbleApp;
use egui::{CentralPanel, Context, TopBottomPanel, Widget};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct LoginView {
    #[serde(skip, default)]
    login_register: LoginRegisterState,
}

impl LoginView {
    fn show_top_bar(&mut self, app: &mut WabbleApp, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Wabble");

            ui.separator();

            ui.label("Login");

            ui.separator();

            ConnectionWindow::new(&mut app.ws, &mut app.windows.connection_window)
                .toggle_button(ui)
                .show(ui.ctx());
        });
    }

    fn show_centered(&mut self, app: &mut WabbleApp, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.set_max_width(ui.available_width() / 2.0);
            ui.add_space(ui.available_height() / 8.0);

            LoginRegister::new(&mut self.login_register, &mut app.ws, &mut app.toasts).ui(ui);
        });
    }
}

impl View for LoginView {
    fn update(&mut self, app: &mut WabbleApp, ctx: &Context) {
        TopBottomPanel::top("login_view_top").show(ctx, |ui| {
            self.show_top_bar(app, ui);
        });

        CentralPanel::default().show(ctx, |ui| {
            self.show_centered(app, ui);
        });
    }
}
