use crate::database::entity;
use wabble_core::types::user_permissions::UserPermissions;

impl entity::user::Model {
    pub fn permissions(&self) -> UserPermissions {
        UserPermissions::from_bits_truncate(self.permissions)
    }

    pub fn has_permissions(&self, permissions: UserPermissions) -> bool {
        self.permissions().has_permissions(permissions)
    }
}
