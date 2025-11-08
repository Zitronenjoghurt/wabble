use crate::database::entity;
use wabble_core::types::friendship_status::FriendshipStatus;

impl entity::user_friendship::Model {
    pub fn status(&self) -> FriendshipStatus {
        FriendshipStatus::from(self.status)
    }
}
