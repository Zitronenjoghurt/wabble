use crate::systems::ws::WebsocketClient;
use crate::windows::{AppWindow, ToggleableWindow};
use egui::{Id, TextEdit, Ui, WidgetText};
use egui_phosphor::regular;
use serde::{Deserialize, Serialize};
use wabble_core::message::client::ClientMessage;

#[derive(Default, Serialize, Deserialize)]
pub struct SendFriendRequestWindowState {
    pub is_open: bool,
    friend_code: String,
}

pub struct SendFriendRequestWindow<'a> {
    ws: &'a mut WebsocketClient,
    state: &'a mut SendFriendRequestWindowState,
}

impl<'a> SendFriendRequestWindow<'a> {
    pub fn new(ws: &'a mut WebsocketClient, state: &'a mut SendFriendRequestWindowState) -> Self {
        Self { ws, state }
    }
}

impl AppWindow for SendFriendRequestWindow<'_> {
    fn id() -> egui::Id {
        Id::new("send_friend_request_window")
    }

    fn title() -> impl Into<WidgetText> {
        "Send Friend request"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        ui.add(
            TextEdit::singleline(&mut self.state.friend_code)
                .char_limit(12)
                .hint_text("Friend code"),
        );

        ui.separator();

        if self.state.friend_code.len() < 12 {
            ui.small("Enter a valid friend code");
        } else if ui.button("Submit").clicked() {
            let _ = self.ws.send(ClientMessage::SendFriendRequest {
                friend_code: self.state.friend_code.clone(),
            });
            self.state.friend_code.clear();
            self.set_open(false);
        }
    }
}

impl ToggleableWindow for SendFriendRequestWindow<'_> {
    fn toggle_label(&self) -> &'static str {
        regular::USER_PLUS
    }
}
