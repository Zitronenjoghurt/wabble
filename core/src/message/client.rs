use bincode::{Decode, Encode};

#[derive(Debug, PartialEq, Eq, Encode, Decode)]
pub enum ClientMessage {
    Ping,
    Login {
        username: String,
        password: String,
    },
    Register {
        username: String,
        password: String,
        invite_code: String,
    },
    Admin(ClientAdminCommand),
}

#[derive(Debug, PartialEq, Eq, Encode, Decode)]
pub enum ClientAdminCommand {
    GenerateInviteCodes(u8),
    RetrieveInviteCodes,
}
