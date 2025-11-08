use crate::crypto::secret::Secret;
use bincode::{Decode, Encode};

#[derive(Debug, PartialEq, Eq, Encode, Decode)]
pub enum ClientMessage {
    Ping,
    Login {
        username: String,
        password: Secret,
    },
    LoginSession {
        id: String,
        token: Secret,
    },
    Register {
        username: String,
        password: Secret,
        invite_code: String,
    },
    RequestSessionToken,
    Admin(ClientAdminCommand),
}

#[derive(Debug, PartialEq, Eq, Encode, Decode)]
pub enum ClientAdminCommand {
    GenerateInviteCodes(u8),
    RetrieveInviteCodes,
}
