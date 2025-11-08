use wabble_core::types::me::Me;
use wabble_core::types::user_permissions::UserPermissions;

#[derive(Debug, Default, Clone)]
pub enum AuthState {
    #[default]
    Unauthenticated,
    Authenticated(Me),
}

impl AuthState {
    pub fn clear(&mut self) {
        *self = AuthState::Unauthenticated;
    }

    pub fn set_authenticated(&mut self, me: Me) {
        *self = AuthState::Authenticated(me);
    }

    pub fn is_authenticated(&self) -> bool {
        matches!(self, AuthState::Authenticated(_))
    }

    pub fn me(&self) -> Option<&Me> {
        if let AuthState::Authenticated(me) = self {
            Some(me)
        } else {
            None
        }
    }

    pub fn permissions(&self) -> Option<UserPermissions> {
        if let AuthState::Authenticated(me) = self {
            Some(me.permissions)
        } else {
            None
        }
    }

    pub fn has_administration_permissions(&self) -> bool {
        if let AuthState::Authenticated(me) = self {
            me.permissions.has_permissions(UserPermissions::ADMIN)
                || me
                    .permissions
                    .has_permissions(UserPermissions::INVITE_MANAGER)
        } else {
            false
        }
    }

    pub fn has_permissions(&self, permissions: UserPermissions) -> bool {
        if let AuthState::Authenticated(me) = self {
            me.permissions.has_permissions(permissions)
        } else {
            false
        }
    }
}
