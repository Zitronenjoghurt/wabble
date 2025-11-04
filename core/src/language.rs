use bincode::{Decode, Encode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Encode, Decode)]
pub enum Language {
    English,
    German,
}
