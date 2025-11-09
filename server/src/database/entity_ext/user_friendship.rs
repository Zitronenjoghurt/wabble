use crate::database::entity;
use wabble_core::types::friendship_status::FriendshipStatus;

impl entity::user_friendship::Model {
    pub fn status(&self) -> FriendshipStatus {
        FriendshipStatus::from(self.status)
    }

    pub fn is_requested_from(&self, user_id: &uuid::Uuid) -> bool {
        if self.status() == FriendshipStatus::RequestedFrom1 {
            self.user1_id == *user_id
        } else if self.status() == FriendshipStatus::RequestedFrom2 {
            self.user2_id == *user_id
        } else {
            false
        }
    }

    pub fn is_requested_from_other(&self, user_id: &uuid::Uuid) -> bool {
        if self.status() == FriendshipStatus::RequestedFrom1 {
            self.user2_id == *user_id
        } else if self.status() == FriendshipStatus::RequestedFrom2 {
            self.user1_id == *user_id
        } else {
            false
        }
    }

    pub fn get_other_user_id(&self, user_id: &uuid::Uuid) -> uuid::Uuid {
        if self.user1_id == *user_id {
            self.user2_id
        } else {
            self.user1_id
        }
    }
}
