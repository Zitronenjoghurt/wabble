use crate::systems::ws::WebsocketClient;
use crate::widgets::friend_request::FriendRequestWidget;
use crate::windows::{AppWindow, ToggleableWindow};
use egui::{Grid, Id, ScrollArea, Ui, Widget, WidgetText};
use egui_phosphor::regular;
use serde::{Deserialize, Serialize};
use wabble_core::message::client::ClientMessage;

#[derive(Default, Serialize, Deserialize)]
pub struct FriendRequestsWindowState {
    pub is_open: bool,
}

pub struct FriendRequestsWindow<'a> {
    ws: &'a mut WebsocketClient,
    state: &'a mut FriendRequestsWindowState,
}

impl<'a> FriendRequestsWindow<'a> {
    pub fn new(ws: &'a mut WebsocketClient, state: &'a mut FriendRequestsWindowState) -> Self {
        Self { ws, state }
    }
}

impl AppWindow for FriendRequestsWindow<'_> {
    fn id() -> Id {
        Id::new("friend_requests_window")
    }

    fn title() -> impl Into<WidgetText> {
        "Friend Requests"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        if self.ws.store().friend_requests.is_empty() {
            ui.small("No friend requests");
        } else {
            ScrollArea::vertical().show(ui, |ui| {
                Grid::new("friend_requests_grid")
                    .num_columns(1)
                    .striped(true)
                    .show(ui, |ui| {
                        let friend_requests = self.ws.store().friend_requests.clone();
                        for friend_request in friend_requests {
                            let mut accepted = false;
                            let mut blocked = false;
                            FriendRequestWidget::new(&friend_request, &mut accepted, &mut blocked)
                                .ui(ui);
                            if accepted {
                                let _ = self.ws.send(ClientMessage::AcceptFriendRequest {
                                    user_id: friend_request.user_id.clone(),
                                });
                            }
                            if blocked {
                                let _ = self.ws.send(ClientMessage::BlockFriendRequest {
                                    user_id: friend_request.user_id.clone(),
                                });
                            }
                            if accepted || blocked {
                                self.ws
                                    .store_mut()
                                    .remove_friend_request(&friend_request.user_id);
                            }
                        }
                    });
            });
        }
    }
}

impl ToggleableWindow for FriendRequestsWindow<'_> {
    fn toggle_label(&self) -> String {
        let icon = if self.is_open() {
            regular::ENVELOPE_OPEN
        } else {
            regular::ENVELOPE
        };

        if self.ws.store().friend_requests.is_empty() {
            icon.to_string()
        } else {
            format!("{} {}!", icon, self.ws.store().friend_requests.len())
        }
    }
}
