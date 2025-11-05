use bincode::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
pub enum ServerMessage {
    Admin(ServerAdminMessage),
}

#[derive(Debug, Clone, Encode, Decode)]
pub enum ServerAdminMessage {
    InviteCodes(Vec<String>),
}
