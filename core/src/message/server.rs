use bincode::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum ServerMessage {
    Pong,
    Error(ServerError),
    LoginSuccess,
    AlreadyLoggedIn,
    Admin(ServerAdminMessage),
}

pub type ServerResult<T> = Result<T, ServerError>;
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, thiserror::Error)]
pub enum ServerError {
    #[error("Database error")]
    Database,
    #[error("Invalid credentials")]
    InvalidCredentials,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum ServerAdminMessage {
    InviteCodes(Vec<String>),
}
