use std::fmt::{Debug, Display};

use mlua::IntoLua;

use crate::position::Position;

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

    pub fn text(&self) -> &str {
        match self {
            Self::White => "white",
            Self::Black => "black",
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

#[derive(Clone)]
pub struct Piece {
    pub kind: String,
    pub color: PieceColor,
    pub position: Position,
    // pub history: Vec<Position>,
    pub last_moved: Option<usize>,
}

impl Piece {
    pub fn new(kind: String, color: PieceColor, position: Position) -> Self {
        Self {
            kind,
            color,
            position,
            // history: vec![],
            last_moved: None,
        }
    }
}

impl IntoLua for &Piece {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let piece = lua.create_table_with_capacity(0, 4)?;

        piece.set("kind", self.kind.clone())?;
        piece.set("color", self.color.to_string())?;
        piece.set("position", self.position)?;
        piece.set("last_moved", self.last_moved)?;

        Ok(mlua::Value::Table(piece))
    }
}
