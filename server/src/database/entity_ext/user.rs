use crate::database::entity;
use crate::types::user_permissions::UserPermissions;

impl entity::user::Model {
    pub fn permissions(&self) -> UserPermissions {
        UserPermissions::from_bits_truncate(self.permissions)
    }
}
