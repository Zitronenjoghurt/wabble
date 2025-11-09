use crate::windows::admin::AdminWindowState;
use crate::windows::connection::ConnectionWindowState;
use crate::windows::friend_info::{FriendInfoWindow, FriendInfoWindowState};
use crate::windows::friend_requests::FriendRequestsWindowState;
use crate::windows::send_friend_request::SendFriendRequestWindowState;
use crate::windows::AppWindow;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct WindowsSystem {
    pub admin_window: AdminWindowState,
    pub connection_window: ConnectionWindowState,
    pub friend_info_window: FriendInfoWindowState,
    pub friend_requests_window: FriendRequestsWindowState,
    pub send_friend_request_window: SendFriendRequestWindowState,
}

impl WindowsSystem {
    pub fn update(&mut self, app: &mut crate::WabbleApp, ctx: &egui::Context) {
        FriendInfoWindow::new(&mut app.ws, &mut self.friend_info_window).show(ctx);
    }
}
