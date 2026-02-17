use mlua::IntoLua;

use crate::engine::{color::PieceColor, moves::Move};

// pub struct PieceId {
//     mod_id: String,
//     id: String,
// }

#[derive(Clone)]
pub struct Piece {
    pub id: uuid::Uuid,
    pub kind: String,
    pub color: PieceColor,
    pub history: Vec<Move>,
    pub last_moved: Option<usize>,
    pub metadata: mlua::Table,
}

impl Piece {
    pub fn new(kind: String, color: PieceColor, lua: &mlua::Lua) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            kind,
            color,
            history: vec![],
            last_moved: None,
            metadata: lua.create_table().unwrap(),
        }
    }
}

impl IntoLua for &Piece {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let piece = lua.create_table_with_capacity(0, 4)?;

        piece.set("id", self.id.to_string())?;
        piece.set("kind", self.kind.clone())?;
        piece.set("color", self.color.to_string())?;
        piece.set("history", self.history.clone())?;
        piece.set("last_moved", self.last_moved)?;
        piece.set("metadata", &self.metadata)?;

        Ok(mlua::Value::Table(piece))
    }
}
