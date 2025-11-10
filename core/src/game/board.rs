use crate::game::board::bonus::Bonus;
use crate::game::board::line::BoardLines;
use crate::game::error::{GameError, GameResult};
use bincode::{Decode, Encode};
use cell::Cell;
use coordinates::BoardCoords;

pub mod board_move;
pub mod bonus;
pub mod cell;
pub mod coordinates;
pub mod line;
pub mod tile;

#[derive(Debug, Encode, Decode)]
pub struct Board {
    cells: Vec<Cell>,
    size: u8,
}

impl Board {
    pub fn new(size: u8) -> GameResult<Self> {
        let mut cells = Vec::with_capacity((size * size) as usize);
        for _ in 0..(size * size) {
            cells.push(Cell::default());
        }

        let mut board = Self { cells, size };
        bonus::generate_bonuses(&mut board)?;

        Ok(board)
    }

    pub fn size(&self) -> u8 {
        self.size
    }

    pub fn get_cell(&self, coords: &BoardCoords) -> GameResult<&Cell> {
        self.cells
            .get(coords.as_index())
            .ok_or(GameError::InvalidCoordinatesXY {
                x: coords.x(self.size),
                y: coords.y(self.size),
                size: self.size,
            })
    }

    pub fn get_cell_mut(&mut self, coords: &BoardCoords) -> GameResult<&mut Cell> {
        self.cells
            .get_mut(coords.as_index())
            .ok_or(GameError::InvalidCoordinatesXY {
                x: coords.x(self.size),
                y: coords.y(self.size),
                size: self.size,
            })
    }

    pub fn get_tile(&self, coords: &BoardCoords) -> GameResult<tile::Tile> {
        Ok(self.get_cell(coords)?.tile)
    }

    pub fn has_tile(&self, coords: &BoardCoords) -> bool {
        self.get_tile(coords)
            .is_ok_and(|tile| tile != tile::Tile::Empty)
    }

    pub fn get_bonus(&self, coords: &BoardCoords) -> GameResult<Bonus> {
        Ok(self.get_cell(coords)?.bonus)
    }

    pub fn display_bonuses(&self) -> GameResult<String> {
        let mut result = String::new();

        for y in 0..self.size {
            let mut row = String::new();
            for x in 0..self.size {
                let coords = BoardCoords::from_x_y(x, y, self.size)?;
                let bonus = self.get_bonus(&coords)?;
                row.push_str(&bonus.to_string());
            }
            result.push_str(&row);
            result.push('\n');
        }

        Ok(result)
    }

    fn affected_lines(&self, board_move: &board_move::BoardMove) -> BoardLines {
        let mut lines = BoardLines::new();
        for part in &board_move.parts {
            self.affected_lines_part(&part.coords, &mut lines);
        }
        lines.prune_single_lines();
        lines
    }

    fn affected_lines_part(&self, coords: &BoardCoords, lines: &mut BoardLines) {
        self.affected_lines_left(coords, lines);
        self.affected_lines_right(coords, lines);
        self.affected_lines_top(coords, lines);
        self.affected_lines_bottom(coords, lines);
    }

    fn affected_lines_left(&self, coords: &BoardCoords, lines: &mut BoardLines) {
        let (x, y) = coords.as_x_y(self.size);
        lines.extend_horizontal(x, y);
        if let Some(left) = coords.left(self.size)
            && self.has_tile(&left)
        {
            self.affected_lines_left(&left, lines);
        }
    }

    fn affected_lines_right(&self, coords: &BoardCoords, lines: &mut BoardLines) {
        let (x, y) = coords.as_x_y(self.size);
        lines.extend_horizontal(x, y);
        if let Some(right) = coords.right(self.size)
            && self.has_tile(&right)
        {
            self.affected_lines_right(&right, lines);
        }
    }

    fn affected_lines_top(&self, coords: &BoardCoords, lines: &mut BoardLines) {
        let (x, y) = coords.as_x_y(self.size);
        lines.extend_vertical(x, y);
        if let Some(top) = coords.top(self.size)
            && self.has_tile(&top)
        {
            self.affected_lines_top(&top, lines);
        }
    }

    fn affected_lines_bottom(&self, coords: &BoardCoords, lines: &mut BoardLines) {
        let (x, y) = coords.as_x_y(self.size);
        lines.extend_vertical(x, y);
        if let Some(bottom) = coords.bottom(self.size)
            && self.has_tile(&bottom)
        {
            self.affected_lines_bottom(&bottom, lines);
        }
    }

    fn validate_move(
        &self,
        board_move: &board_move::BoardMove,
        lines: &BoardLines,
    ) -> GameResult<()> {
        if !board_move.contained_in_one_line(lines, self.size) {
            return Err(GameError::InvalidMove);
        }

        if board_move
            .parts
            .iter()
            .any(|part| self.has_tile(&part.coords))
        {
            return Err(GameError::InvalidMove);
        }

        Ok(())
    }

    pub fn evaluate(
        &self,
        board_move: &board_move::BoardMove,
    ) -> GameResult<board_move::BoardMoveEvaluation> {
        let lines = self.affected_lines(board_move);
        self.validate_move(board_move, &lines)?;

        todo!()
    }
}
