#[derive(Debug, Clone, Copy)]
pub enum FriendshipStatus {
    None = 0,
    RequestedFrom1 = 1,
    RequestedFrom2 = 2,
    RequestedFromBoth = 3,
    BlockedBy1 = 4,
    BlockedBy2 = 5,
    Accepted = 6,
}

impl From<FriendshipStatus> for i16 {
    fn from(status: FriendshipStatus) -> Self {
        status as i16
    }
}

impl From<i16> for FriendshipStatus {
    fn from(status: i16) -> Self {
        match status {
            1 => FriendshipStatus::RequestedFrom1,
            2 => FriendshipStatus::RequestedFrom2,
            3 => FriendshipStatus::RequestedFromBoth,
            4 => FriendshipStatus::BlockedBy1,
            5 => FriendshipStatus::BlockedBy2,
            6 => FriendshipStatus::Accepted,
            _ => FriendshipStatus::None,
        }
    }
}
