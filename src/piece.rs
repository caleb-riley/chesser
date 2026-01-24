use std::fmt::Display;

use crate::position::Position;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    King,
    Queen,
}

impl PieceKind {
    pub fn label(&self) -> char {
        match self {
            Self::Pawn => 'P',
            Self::Knight => 'N',
            Self::Bishop => 'B',
            Self::Rook => 'R',
            Self::King => 'K',
            Self::Queen => 'Q',
        }
    }

    pub fn value(&self) -> usize {
        match self {
            Self::Pawn => 1,
            Self::Knight => 3,
            Self::Bishop => 3,
            Self::Rook => 5,
            Self::King => 0,
            Self::Queen => 9,
        }
    }
}

impl TryFrom<char> for PieceKind {
    type Error = ();

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        let kind = match value {
            'P' => Self::Pawn,
            'N' => Self::Knight,
            'B' => Self::Bishop,
            'R' => Self::Rook,
            'K' => Self::King,
            'Q' => Self::Queen,
            _ => return Err(()),
        };

        Ok(kind)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn from_turn_count(count: usize) -> Self {
        if count.is_multiple_of(2) {
            Self::White
        } else {
            Self::Black
        }
    }
}

impl Display for PieceColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::White => "White",
            Self::Black => "Black",
        };

        write!(f, "{label}")
    }
}

#[derive(Clone, Debug)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: PieceColor,
    pub position: Position,
    pub previous: Position,
    pub turns: usize,
    pub last_moved: Option<usize>,
}

impl Piece {
    pub fn new(kind: PieceKind, color: PieceColor, position: Position) -> Self {
        Self {
            kind,
            color,
            position,
            previous: position,
            turns: 0,
            last_moved: None,
        }
    }
}
