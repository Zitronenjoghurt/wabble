use crate::crypto::secret::Secret;
use crate::types::friend_info::FriendInfo;
use crate::types::friend_request_info::FriendRequestInfo;
use crate::types::me::Me;
use crate::validate::ValidationError;
use bincode::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum ServerMessage {
    Pong,
    Error(ServerError),
    Authenticated(Me),
    SessionToken { id: String, token: Secret },
    FriendRequestSent,
    FriendRequestAccepted,
    FriendRequestBlocked,
    FriendRequestReceived(FriendRequestInfo),
    FriendRequestWasAccepted(FriendInfo),
    FriendRequests(Vec<FriendRequestInfo>),
    Friends(Vec<FriendInfo>),
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
    #[error("Friend code invalid")]
    FriendCodeInvalid,
    #[error("Friend request already accepted")]
    FriendRequestAlreadyAccepted,
    #[error("Friend request already sent")]
    FriendRequestAlreadySent,
    #[error("Friend request blocked by user")]
    FriendRequestBlocked,
    #[error("No friend request")]
    NoFriendRequest,
    #[error("Session invalid")]
    SessionInvalid,
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
