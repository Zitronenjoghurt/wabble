use crate::windows::admin::AdminWindowState;
use crate::windows::connection::ConnectionWindowState;
use crate::windows::friend_requests::FriendRequestsWindowState;
use crate::windows::send_friend_request::SendFriendRequestWindowState;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct WindowsSystem {
    pub admin_window: AdminWindowState,
    pub connection_window: ConnectionWindowState,
    pub friend_requests_window: FriendRequestsWindowState,
    pub send_friend_request_window: SendFriendRequestWindowState,
}
