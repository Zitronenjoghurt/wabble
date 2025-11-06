use bincode::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum ServerMessage {
    Pong,
    Admin(ServerAdminMessage),
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum ServerAdminMessage {
    InviteCodes(Vec<String>),
}
