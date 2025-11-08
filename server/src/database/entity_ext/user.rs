use crate::database::entity;
use wabble_core::types::me::Me;
use wabble_core::types::user_permissions::UserPermissions;

impl entity::user::Model {
    pub fn permissions(&self) -> UserPermissions {
        UserPermissions::from_bits_truncate(self.permissions)
    }

    pub fn has_permissions(&self, permissions: UserPermissions) -> bool {
        self.permissions().has_permissions(permissions)
    }

    pub fn get_me(&self) -> Me {
        Me {
            username: self.name.clone(),
            permissions: self.permissions(),
            friend_code: self.friend_code.clone(),
        }
    }
}
