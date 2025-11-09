use bincode::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct FriendRequestInfo {
    pub user_id: String,
    pub user_name: String,
}
