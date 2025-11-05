use crate::views::View;
use crate::widgets::login_register::{LoginRegister, LoginRegisterState};
use crate::WabbleApp;
use egui::{CentralPanel, Context, TopBottomPanel, Widget};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct MainMenuView {
    #[serde(skip, default)]
    login_register: LoginRegisterState,
}

impl MainMenuView {
    fn show_centered(&mut self, app: &mut WabbleApp, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.set_max_width(ui.available_width() / 2.0);
            ui.add_space(ui.available_height() / 8.0);

            LoginRegister::new(&mut self.login_register, &mut app.ws).ui(ui);
        });
    }
}

impl View for MainMenuView {
    fn update(&mut self, app: &mut WabbleApp, ctx: &Context) {
        TopBottomPanel::top("main_menu_top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Wabble");

                ui.separator();

                ui.label("Login")
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            self.show_centered(app, ui);
        });
    }
}
