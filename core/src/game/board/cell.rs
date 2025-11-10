use crate::game::board::bonus::Bonus;
use crate::game::board::tile::Tile;
use bincode::{Decode, Encode};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub struct Cell {
    pub tile: Tile,
    pub bonus: Bonus,
    pub value: u8,
}
