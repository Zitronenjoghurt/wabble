use crate::game::board::coordinates::BoardCoords;
use crate::game::board::line::BoardLine;
use crate::game::board::line::BoardLines;
use crate::game::board::tile::Tile;
use bincode::{Decode, Encode};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Encode, Decode)]
pub struct BoardMove {
    pub parts: Vec<BoardMovePart>,
}

impl BoardMove {
    pub fn new(parts: Vec<BoardMovePart>) -> Self {
        Self { parts }
    }

    pub fn contained_in_one_line(&self, lines: &BoardLines, board_size: u8) -> bool {
        for line in lines.vertical_lines() {
            if self
                .parts
                .iter()
                .all(|part| line.contains(part.coords.x(board_size), part.coords.y(board_size)))
            {
                return true;
            }
        }

        for line in lines.horizontal_lines() {
            if self
                .parts
                .iter()
                .all(|part| line.contains(part.coords.x(board_size), part.coords.y(board_size)))
            {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Encode, Decode)]
pub struct BoardMovePart {
    pub coords: BoardCoords,
    pub tile: Tile,
}

impl BoardMovePart {
    pub fn new(coords: BoardCoords, tile: Tile) -> Self {
        Self { coords, tile }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Encode, Decode)]
pub struct BoardMoveEvaluation {
    pub words: Vec<String>,
    pub score: u32,
}
