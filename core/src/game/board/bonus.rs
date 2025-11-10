use crate::game::board::coordinates::BoardCoords;
use crate::game::board::Board;
use crate::game::error::{GameError, GameResult};
use bincode::{Decode, Encode};
use std::fmt::{Display, Formatter};

mod quadrants;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum Bonus {
    #[default]
    None = 0,
    Anchor = 1,
    DL = 2,
    DW = 3,
    TL = 4,
    TW = 5,
}

impl Display for Bonus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Bonus::None => write!(f, "[]"),
            Bonus::Anchor => write!(f, "XX"),
            Bonus::DL => write!(f, "DL"),
            Bonus::DW => write!(f, "DW"),
            Bonus::TL => write!(f, "TL"),
            Bonus::TW => write!(f, "TW"),
        }
    }
}

pub fn generate_bonuses(board: &mut Board) -> GameResult<()> {
    let quadrant = match board.size {
        15 => Ok(quadrants::QUADRANT_15X15),
        _ => Err(GameError::UnsupportedBoardSize(board.size)),
    }?;

    for (x, y, bonus) in quadrant {
        let coords = BoardCoords::from_x_y(*x, *y, board.size)?;
        apply_bonus(board, coords, *bonus)?;
    }

    Ok(())
}

fn apply_bonus(board: &mut Board, coords_up_left: BoardCoords, bonus: Bonus) -> GameResult<()> {
    let coords_up_right = coords_up_left.mirror_vertically(board.size)?;
    let coords_down_left = coords_up_left.mirror_horizontally(board.size)?;
    let coords_down_right = coords_up_right.mirror_horizontally(board.size)?;
    board.get_cell_mut(&coords_up_left)?.bonus = bonus;
    board.get_cell_mut(&coords_up_right)?.bonus = bonus;
    board.get_cell_mut(&coords_down_left)?.bonus = bonus;
    board.get_cell_mut(&coords_down_right)?.bonus = bonus;
    Ok(())
}
