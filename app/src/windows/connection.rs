use crate::systems::ws::WebsocketClient;
use crate::widgets::connection_status::ConnectionStatus;
use crate::windows::{AppWindow, ToggleableWindow};
use egui::{Id, Ui, Widget, WidgetText};
use egui_phosphor::regular;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct ConnectionWindowState {
    pub is_open: bool,
    pub url: String,
}

pub struct ConnectionWindow<'a> {
    ws: &'a mut WebsocketClient,
    state: &'a mut ConnectionWindowState,
}

impl<'a> ConnectionWindow<'a> {
    pub fn new(ws: &'a mut WebsocketClient, state: &'a mut ConnectionWindowState) -> Self {
        Self { ws, state }
    }
}

impl AppWindow for ConnectionWindow<'_> {
    fn id() -> Id {
        Id::new("connection_window")
    }

    fn title() -> impl Into<WidgetText> {
        "Connection"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        ConnectionStatus::new(self.ws, &mut self.state.url).ui(ui);
    }
}

impl ToggleableWindow for ConnectionWindow<'_> {
    fn toggle_label(&self) -> String {
        if self.ws.is_connected() {
            regular::CELL_SIGNAL_FULL
        } else {
            regular::CELL_SIGNAL_X
        }
        .to_string()
    }
}
