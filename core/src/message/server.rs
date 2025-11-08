use crate::types::user_permissions::UserPermissions;
use crate::validate::ValidationError;
use bincode::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum ServerMessage {
    Pong,
    Error(ServerError),
    LoginSuccess(UserPermissions),
    AlreadyLoggedIn(UserPermissions),
    Admin(ServerAdminMessage),
}

pub type ServerResult<T> = Result<T, ServerError>;
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, thiserror::Error)]
pub enum ServerError {
    #[error("Database error")]
    Database,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid invite code")]
    InvalidInviteCode,
    #[error("Forbidden")]
    Forbidden,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Unexpected error")]
    Unexpected,
    #[error("Username taken")]
    UsernameTaken,
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum ServerAdminMessage {
    InviteCodes(Vec<String>),
}
