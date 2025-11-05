use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub enum ClientMessage {
    Admin(ClientAdminMessage),
}

#[derive(Debug, Encode, Decode)]
pub struct ClientAdminMessage {
    token: String,
    command: ClientAdminCommand,
}

#[derive(Debug, Encode, Decode)]
pub enum ClientAdminCommand {
    GenerateInviteCodes(u8),
    RetrieveInviteCodes,
}
