use bincode::{Decode, Encode};

#[derive(Debug, PartialEq, Eq, Encode, Decode)]
pub enum ClientMessage {
    Ping,
    Admin(ClientAdminMessage),
}

#[derive(Debug, PartialEq, Eq, Encode, Decode)]
pub struct ClientAdminMessage {
    token: String,
    command: ClientAdminCommand,
}

#[derive(Debug, PartialEq, Eq, Encode, Decode)]
pub enum ClientAdminCommand {
    GenerateInviteCodes(u8),
    RetrieveInviteCodes,
}
