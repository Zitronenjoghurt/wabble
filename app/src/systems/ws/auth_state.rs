use wabble_core::types::user_permissions::UserPermissions;

#[derive(Debug, Default, Clone, Copy)]
pub enum AuthState {
    #[default]
    Unauthenticated,
    Authenticated(UserPermissions),
}

impl AuthState {
    pub fn clear(&mut self) {
        *self = AuthState::Unauthenticated;
    }

    pub fn set_authenticated(&mut self, permissions: UserPermissions) {
        *self = AuthState::Authenticated(permissions);
    }

    pub fn is_authenticated(&self) -> bool {
        matches!(self, AuthState::Authenticated(_))
    }

    pub fn permissions(&self) -> Option<UserPermissions> {
        if let AuthState::Authenticated(permissions) = self {
            Some(*permissions)
        } else {
            None
        }
    }

    pub fn has_administration_permissions(&self) -> bool {
        if let AuthState::Authenticated(permissions) = self {
            permissions.has_permissions(UserPermissions::ADMIN)
                || permissions.has_permissions(UserPermissions::INVITE_MANAGER)
        } else {
            false
        }
    }

    pub fn has_permissions(&self, permissions: UserPermissions) -> bool {
        if let AuthState::Authenticated(auth_permissions) = self {
            auth_permissions.has_permissions(permissions)
        } else {
            false
        }
    }
}
