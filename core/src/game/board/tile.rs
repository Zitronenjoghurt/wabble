use crate::game::error::{GameError, GameResult};
use bincode::{Decode, Encode};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Encode, Decode)]
pub enum Tile {
    #[default]
    Empty = 0,
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    G = 7,
    H = 8,
    I = 9,
    J = 10,
    K = 11,
    L = 12,
    M = 13,
    N = 14,
    O = 15,
    P = 16,
    Q = 17,
    R = 18,
    S = 19,
    T = 20,
    U = 21,
    V = 22,
    W = 23,
    X = 24,
    Y = 25,
    Z = 26,
    AE = 27,
    OE = 28,
    UE = 29,
    SZ = 30,
}

impl TryFrom<Tile> for char {
    type Error = GameError;

    fn try_from(value: Tile) -> GameResult<Self> {
        match value {
            Tile::A => Ok('A'),
            Tile::B => Ok('B'),
            Tile::C => Ok('C'),
            Tile::D => Ok('D'),
            Tile::E => Ok('E'),
            Tile::F => Ok('F'),
            Tile::G => Ok('G'),
            Tile::H => Ok('H'),
            Tile::I => Ok('I'),
            Tile::J => Ok('J'),
            Tile::K => Ok('K'),
            Tile::L => Ok('L'),
            Tile::M => Ok('M'),
            Tile::N => Ok('N'),
            Tile::O => Ok('O'),
            Tile::P => Ok('P'),
            Tile::Q => Ok('Q'),
            Tile::R => Ok('R'),
            Tile::S => Ok('S'),
            Tile::T => Ok('T'),
            Tile::U => Ok('U'),
            Tile::V => Ok('V'),
            Tile::W => Ok('W'),
            Tile::X => Ok('X'),
            Tile::Y => Ok('Y'),
            Tile::Z => Ok('Z'),
            Tile::AE => Ok('Ä'),
            Tile::OE => Ok('Ö'),
            Tile::UE => Ok('Ü'),
            Tile::SZ => Ok('ẞ'),
            Tile::Empty => Err(GameError::TileIsEmpty),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = GameError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' | 'A' => Ok(Tile::A),
            'b' | 'B' => Ok(Tile::B),
            'c' | 'C' => Ok(Tile::C),
            'd' | 'D' => Ok(Tile::D),
            'e' | 'E' => Ok(Tile::E),
            'f' | 'F' => Ok(Tile::F),
            'g' | 'G' => Ok(Tile::G),
            'h' | 'H' => Ok(Tile::H),
            'i' | 'I' => Ok(Tile::I),
            'j' | 'J' => Ok(Tile::J),
            'k' | 'K' => Ok(Tile::K),
            'l' | 'L' => Ok(Tile::L),
            'm' | 'M' => Ok(Tile::M),
            'n' | 'N' => Ok(Tile::N),
            'o' | 'O' => Ok(Tile::O),
            'p' | 'P' => Ok(Tile::P),
            'q' | 'Q' => Ok(Tile::Q),
            'r' | 'R' => Ok(Tile::R),
            's' | 'S' => Ok(Tile::S),
            't' | 'T' => Ok(Tile::T),
            'u' | 'U' => Ok(Tile::U),
            'v' | 'V' => Ok(Tile::V),
            'w' | 'W' => Ok(Tile::W),
            'x' | 'X' => Ok(Tile::X),
            'y' | 'Y' => Ok(Tile::Y),
            'z' | 'Z' => Ok(Tile::Z),
            'ä' | 'Ä' => Ok(Tile::AE),
            'ö' | 'Ö' => Ok(Tile::OE),
            'ü' | 'Ü' => Ok(Tile::UE),
            'ß' | 'ẞ' => Ok(Tile::SZ),
            _ => Err(GameError::InvalidTileChar(value)),
        }
    }
}
