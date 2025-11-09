use crate::systems::ws::WebsocketClient;
use crate::widgets::friend_info::FriendInfoWidget;
use crate::windows::AppWindow;
use egui::{Id, Ui, Widget, WidgetText};
use serde::{Deserialize, Serialize};
use wabble_core::message::client::ClientMessage;

#[derive(Default, Serialize, Deserialize)]
pub struct FriendInfoWindowState {
    pub friend_id: Option<String>,
}

pub struct FriendInfoWindow<'a> {
    ws: &'a mut WebsocketClient,
    state: &'a mut FriendInfoWindowState,
}

impl<'a> FriendInfoWindow<'a> {
    pub fn new(ws: &'a mut WebsocketClient, state: &'a mut FriendInfoWindowState) -> Self {
        Self { ws, state }
    }
}

impl AppWindow for FriendInfoWindow<'_> {
    fn id() -> Id {
        Id::new("friend_info_window")
    }

    fn title() -> impl Into<WidgetText> {
        "Friend info"
    }

    fn is_open(&self) -> bool {
        self.state.friend_id.is_some()
    }

    fn set_open(&mut self, open: bool) {
        if !open {
            self.state.friend_id = None;
        }
    }

    fn render_content(&mut self, ui: &mut Ui) {
        let Some(friend_id) = &self.state.friend_id else {
            return;
        };

        let Some(info) = self.ws.store().friends.get(friend_id) else {
            self.set_open(false);
            return;
        };

        let mut removed = false;
        FriendInfoWidget::new(info, &mut removed).ui(ui);
        if removed {
            let _ = self.ws.send(ClientMessage::RemoveFriend {
                user_id: friend_id.clone(),
            });
            self.set_open(false);
        }
    }
}
