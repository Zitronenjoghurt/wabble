use crate::systems::ws::WebsocketClient;
use crate::widgets::profile::ProfileWidget;
use crate::windows::{AppWindow, ToggleableWindow};
use egui::{Id, Ui, Widget, WidgetText};
use egui_phosphor::regular;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct ProfileWindowState {
    is_open: bool,
}

pub struct ProfileWindow<'a> {
    ws: &'a mut WebsocketClient,
    state: &'a mut ProfileWindowState,
}

impl<'a> ProfileWindow<'a> {
    pub fn new(ws: &'a mut WebsocketClient, state: &'a mut ProfileWindowState) -> Self {
        Self { ws, state }
    }
}

impl AppWindow for ProfileWindow<'_> {
    fn id() -> Id {
        Id::new("profile_window")
    }

    fn title() -> impl Into<WidgetText> {
        "Profile"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        ProfileWidget::new(self.ws).ui(ui);
    }
}

impl ToggleableWindow for ProfileWindow<'_> {
    fn toggle_label(&self) -> String {
        regular::USER_CIRCLE.to_string()
    }
}
