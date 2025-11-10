use crate::game::board::Board;
use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub struct GameState {
    pub player_1_id: String,
    pub player_2_id: String,
    pub board: Board,
}
