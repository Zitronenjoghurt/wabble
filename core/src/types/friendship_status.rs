use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct FriendshipStatus: i16 {
        const PENDING = 0b1;
        const ACCEPTED = 0b10;
        const BLOCKED = 0b100;
    }
}
