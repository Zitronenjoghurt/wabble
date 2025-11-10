pub type GameResult<T> = Result<T, GameError>;

#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("Tile is empty")]
    TileIsEmpty,
    #[error("Invalid coordinates index for a board size of {size}: {index}")]
    InvalidCoordinatesIndex { index: u16, size: u8 },
    #[error("Invalid coordinates for a board size of {size}: ({x}, {y})")]
    InvalidCoordinatesXY { x: u8, y: u8, size: u8 },
    #[error("Invalid move")]
    InvalidMove,
    #[error("Invalid tile char: {0}")]
    InvalidTileChar(char),
    #[error("Unsupported board size: {0}")]
    UnsupportedBoardSize(u8),
}
