//! The upper left quadrants of each supported board size.

use crate::game::board::bonus::Bonus;

pub static QUADRANT_15X15: &[(u8, u8, Bonus)] = &[
    (0, 0, Bonus::TW),
    (3, 0, Bonus::DL),
    (7, 0, Bonus::TW),
    (1, 1, Bonus::DW),
    (5, 1, Bonus::TL),
    (2, 2, Bonus::DW),
    (6, 2, Bonus::DL),
    (0, 3, Bonus::DL),
    (3, 3, Bonus::DW),
    (7, 3, Bonus::DL),
    (4, 4, Bonus::DW),
    (1, 5, Bonus::TL),
    (5, 5, Bonus::TL),
    (2, 6, Bonus::DL),
    (6, 6, Bonus::DL),
    (0, 7, Bonus::TW),
    (3, 7, Bonus::DL),
    (7, 7, Bonus::Anchor),
];
