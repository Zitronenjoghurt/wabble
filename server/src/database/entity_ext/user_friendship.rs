use crate::database::entity;
use wabble_core::types::friendship_status::FriendshipStatus;

impl entity::user_friendship::Model {
    pub fn status(&self) -> FriendshipStatus {
        FriendshipStatus::from_bits_truncate(self.status)
    }

    pub fn is_pending(&self) -> bool {
        self.status() == FriendshipStatus::PENDING
    }

    pub fn is_accepted(&self) -> bool {
        self.status() == FriendshipStatus::ACCEPTED
    }

    pub fn is_blocked(&self) -> bool {
        self.status() == FriendshipStatus::BLOCKED
    }
}
