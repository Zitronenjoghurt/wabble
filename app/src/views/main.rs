use crate::views::View;
use crate::windows::admin::AdminWindow;
use crate::windows::connection::ConnectionWindow;
use crate::windows::profile::{ProfileWindow, ProfileWindowState};
use crate::windows::send_friend_request::SendFriendRequestWindow;
use crate::windows::{AppWindow, ToggleableWindow};
use crate::WabbleApp;
use egui::{Context, SidePanel, TopBottomPanel};
use egui_phosphor::regular;
use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum RightPanelTab {
    #[default]
    Friends,
}

#[derive(Default, Serialize, Deserialize)]
pub struct MainView {
    profile_window: ProfileWindowState,
    right_panel_tab: RightPanelTab,
}

impl MainView {
    fn show_top_bar(&mut self, app: &mut WabbleApp, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Wabble");

            ui.separator();

            ConnectionWindow::new(&mut app.ws, &mut app.windows.connection_window)
                .toggle_button(ui)
                .show(ui.ctx());

            if app.ws.auth_state().is_authenticated() {
                ProfileWindow::new(&mut app.ws, &mut self.profile_window)
                    .toggle_button(ui)
                    .show(ui.ctx());
            }

            if app.ws.auth_state().has_administration_permissions() {
                AdminWindow::new(app).toggle_button(ui).show(ui.ctx());
            }
        });
    }

    fn show_right_panel(&mut self, app: &mut WabbleApp, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.right_panel_tab,
                RightPanelTab::Friends,
                regular::USERS,
            );
        });

        ui.separator();

        match self.right_panel_tab {
            RightPanelTab::Friends => self.show_friends(app, ui),
        }
    }

    fn show_friends(&mut self, app: &mut WabbleApp, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            SendFriendRequestWindow::new(&mut app.ws, &mut app.windows.send_friend_request_window)
                .toggle_button(ui)
                .show(ui.ctx());
        });

        ui.separator();
    }
}

impl View for MainView {
    fn update(&mut self, app: &mut WabbleApp, ctx: &Context) {
        TopBottomPanel::top("main_view_top").show(ctx, |ui| {
            self.show_top_bar(app, ui);
        });

        SidePanel::right("main_view_right_panel").show(ctx, |ui| {
            self.show_right_panel(app, ui);
        });
    }
}
