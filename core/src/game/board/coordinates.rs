use crate::game::error::{GameError, GameResult};
use bincode::{Decode, Encode};

/// Board coordinates start in the upper left corner.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Encode, Decode)]
#[repr(transparent)]
pub struct BoardCoords(u16);

impl BoardCoords {
    pub fn from_x_y(x: u8, y: u8, size: u8) -> GameResult<Self> {
        if size == 0 {
            return Err(GameError::UnsupportedBoardSize(size));
        }

        if x > size - 1 || y > size - 1 {
            return Err(GameError::InvalidCoordinatesXY { x, y, size });
        }

        Ok(Self(x as u16 + (y as u16 * size as u16)))
    }

    pub fn from_index(index: u16, size: u8) -> GameResult<Self> {
        if size == 0 {
            return Err(GameError::UnsupportedBoardSize(size));
        }

        if index > size as u16 * size as u16 - 1 {
            return Err(GameError::InvalidCoordinatesIndex { index, size });
        }

        Ok(Self(index))
    }

    pub fn x(&self, size: u8) -> u8 {
        (self.0 % size as u16) as u8
    }

    pub fn y(&self, size: u8) -> u8 {
        (self.0 / size as u16) as u8
    }

    pub fn as_x_y(&self, size: u8) -> (u8, u8) {
        (self.x(size), self.y(size))
    }

    pub fn as_index(&self) -> usize {
        self.0 as usize
    }

    pub fn mirror_vertically(&self, size: u8) -> GameResult<Self> {
        let (x, y) = self.as_x_y(size);
        Self::from_x_y(size - x - 1, y, size)
    }

    pub fn mirror_horizontally(&self, size: u8) -> GameResult<Self> {
        let (x, y) = self.as_x_y(size);
        Self::from_x_y(x, size - y - 1, size)
    }

    pub fn top(&self, size: u8) -> Option<Self> {
        if self.y(size) == 0 {
            None
        } else {
            Some(Self(self.0 - size as u16))
        }
    }

    pub fn bottom(&self, size: u8) -> Option<Self> {
        if self.y(size) >= size - 1 {
            None
        } else {
            Some(Self(self.0 + size as u16))
        }
    }

    pub fn left(&self, size: u8) -> Option<Self> {
        if self.x(size) == 0 {
            None
        } else {
            Some(Self(self.0 - 1))
        }
    }

    pub fn right(&self, size: u8) -> Option<Self> {
        if self.x(size) >= size - 1 {
            None
        } else {
            Some(Self(self.0 + 1))
        }
    }

    pub fn adjacent(&self, size: u8) -> Vec<Self> {
        let mut result = Vec::new();
        if let Some(top) = self.top(size) {
            result.push(top);
        }
        if let Some(bottom) = self.bottom(size) {
            result.push(bottom);
        }
        if let Some(left) = self.left(size) {
            result.push(left);
        }
        if let Some(right) = self.right(size) {
            result.push(right);
        }
        result
    }
}
