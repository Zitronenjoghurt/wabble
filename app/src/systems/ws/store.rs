use crate::types::timeout::Timeout;
use egui::ahash::HashMap;
use wabble_core::types::friend_info::FriendInfo;
use wabble_core::types::friend_request_info::FriendRequestInfo;

pub struct WsStore {
    pub invite_codes: Vec<String>,
    pub timer_friendship: Timeout,
    pub friends: HashMap<String, FriendInfo>,
    pub friend_requests: Vec<FriendRequestInfo>,
}

impl Default for WsStore {
    fn default() -> Self {
        Self {
            invite_codes: Vec::default(),
            timer_friendship: Timeout::from_secs(30),
            friends: HashMap::default(),
            friend_requests: Vec::default(),
        }
    }
}

impl WsStore {
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn remove_friend_request(&mut self, user_id: &str) {
        self.friend_requests
            .retain(|request| request.user_id != user_id);
    }
}
