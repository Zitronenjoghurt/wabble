use crate::views::View;
use crate::windows::admin::AdminWindow;
use crate::windows::connection::ConnectionWindow;
use crate::windows::{AppWindow, ToggleableWindow};
use crate::WabbleApp;
use egui::{Context, TopBottomPanel};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct MainView;

impl MainView {
    fn show_top_bar(&mut self, app: &mut WabbleApp, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Wabble");

            ui.separator();

            ConnectionWindow::new(&mut app.ws, &mut app.windows.connection_window)
                .toggle_button(ui)
                .show(ui.ctx());

            if app.ws.auth_state().has_administration_permissions() {
                AdminWindow::new(app).toggle_button(ui).show(ui.ctx());
            }
        });
    }
}

impl View for MainView {
    fn update(&mut self, app: &mut WabbleApp, ctx: &Context) {
        TopBottomPanel::top("main_view_top").show(ctx, |ui| {
            self.show_top_bar(app, ui);
        });
    }
}
