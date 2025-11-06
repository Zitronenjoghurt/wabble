use bitflags::bitflags;

bitflags! {
    pub struct UserPermissions: i64 {
        const ADMIN = 0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64 as i64;
    }
}

impl Default for UserPermissions {
    fn default() -> Self {
        Self::empty()
    }
}
