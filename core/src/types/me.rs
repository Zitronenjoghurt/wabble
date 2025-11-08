use crate::types::user_permissions::UserPermissions;
use bincode::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct Me {
    pub username: String,
    pub permissions: UserPermissions,
    pub friend_code: String,
}
