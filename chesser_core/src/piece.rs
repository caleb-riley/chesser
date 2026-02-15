use std::fmt::{Debug, Display};

use mlua::IntoLua;

use crate::{board::Board, moves::Move, position::Position};

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
            Self::White => "white",
            Self::Black => "black",
        };

        write!(f, "{label}")
    }
}

impl mlua::FromLua for PieceColor {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::String(color) = value else {
            return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "string".to_string(),
                message: Some("expected string".into()),
            });
        };

        match color.to_string_lossy().as_str() {
            "white" => Ok(Self::White),
            "black" => Ok(Self::Black),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "string",
                to: "PieceColor".to_string(),
                message: Some("expected valid PieceColor".into()),
            }),
        }
    }
}

impl IntoLua for PieceColor {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        match self {
            Self::White => Ok("white".into_lua(lua)?),
            Self::Black => Ok("black".into_lua(lua)?),
        }
    }
}

#[derive(Clone)]
pub struct Piece {
    pub kind: String,
    pub color: PieceColor,
    pub history: Vec<Move>,
    pub last_moved: Option<usize>,
}

impl Piece {
    pub fn new(kind: String, color: PieceColor) -> Self {
        Self {
            kind,
            color,
            history: vec![],
            last_moved: None,
        }
    }
}

impl IntoLua for &Piece {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let piece = lua.create_table_with_capacity(0, 4)?;

        piece.set("kind", self.kind.clone())?;
        piece.set("color", self.color.to_string())?;
        piece.set("history", self.history.clone())?;
        piece.set("last_moved", self.last_moved)?;

        Ok(mlua::Value::Table(piece))
    }
}

pub struct PieceConfig {
    value: i32,
    available_moves: mlua::Function,
}

impl PieceConfig {
    pub fn from_script(lua: &mlua::Lua, id: &str) -> Self {
        let script_path = format!("./lua/pieces/{}.lua", id);
        let script = std::fs::read_to_string(script_path).unwrap();

        let piece_data: mlua::Table = lua.load(script).eval().unwrap();

        let value: mlua::Integer = piece_data.get("value").unwrap();
        let available_moves: mlua::Function = piece_data.get("available_moves").unwrap();

        Self {
            value: value as i32,
            available_moves,
        }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn get_available_moves<'a>(
        &self,
        lua: &'a mlua::Lua,
        board: &'a Board,
        position: Position,
    ) -> Vec<Move> {
        let board_userdata = lua.create_userdata(board.clone()).unwrap();
        let piece = board.get_piece(position).unwrap();

        self.available_moves
            .call((board_userdata, piece, position))
            .unwrap()
    }
}
