use crate::windows::{AppWindow, ToggleableWindow};
use crate::WabbleApp;
use egui::{Ui, WidgetText};
use egui_phosphor::regular;
use serde::{Deserialize, Serialize};
use wabble_core::types::user_permissions::UserPermissions;

mod invites;

#[derive(Default, PartialEq, Serialize, Deserialize)]
pub enum AdminWindowTab {
    #[default]
    General,
    Invites,
}

#[derive(Default, Serialize, Deserialize)]
pub struct AdminWindowState {
    pub is_open: bool,
    pub current_tab: AdminWindowTab,
}

pub struct AdminWindow<'a> {
    app: &'a mut WabbleApp,
}

impl<'a> AdminWindow<'a> {
    pub fn new(app: &'a mut WabbleApp) -> Self {
        Self { app }
    }

    fn state(&self) -> &AdminWindowState {
        &self.app.windows.admin_window
    }

    fn state_mut(&mut self) -> &mut AdminWindowState {
        &mut self.app.windows.admin_window
    }

    fn render_tab_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.state_mut().current_tab,
                AdminWindowTab::General,
                regular::GEAR_SIX,
            );

            if self
                .app
                .ws
                .auth_state()
                .has_permissions(UserPermissions::INVITE_MANAGER)
            {
                ui.selectable_value(
                    &mut self.state_mut().current_tab,
                    AdminWindowTab::Invites,
                    regular::TICKET,
                );
            }
        });
    }

    fn render_tab(&mut self, ui: &mut Ui) {
        match self.state().current_tab {
            AdminWindowTab::General => {}
            AdminWindowTab::Invites => invites::render_invites_tab(self.app, ui),
        }
    }
}

impl AppWindow for AdminWindow<'_> {
    fn id() -> egui::Id {
        egui::Id::new("admin_window")
    }

    fn title() -> impl Into<WidgetText> {
        "Admin"
    }

    fn is_open(&self) -> bool {
        self.state().is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state_mut().is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            self.render_tab_bar(ui);
            ui.separator();
            self.render_tab(ui);
        });
    }
}

impl ToggleableWindow for AdminWindow<'_> {
    fn toggle_label(&self) -> &'static str {
        regular::SHIELD
    }
}
