use bincode::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct FriendInfo {
    pub user_id: String,
    pub user_name: String,
    pub timestamp_utc: i64,
    pub is_online: bool,
}
