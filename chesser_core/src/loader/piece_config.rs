use std::collections::HashMap;

use crate::engine::{board::Board, moves::Move, position::Position};

pub struct PieceConfig {
    value: i32,
    available_moves: mlua::Function,
}

impl PieceConfig {
    pub fn from_path(lua: &mlua::Lua, piece_path: &str) -> Self {
        let script_path = format!("{piece_path}/piece.lua");
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

    pub fn get_available_moves(
        &self,
        lua: &mlua::Lua,
        board: &Board,
        position: Position,
    ) -> mlua::Result<Vec<Move>> {
        let board_userdata = lua.create_userdata(board.clone()).unwrap();
        let piece = board.get_piece_at_position(position).unwrap();

        self.available_moves.call((board_userdata, piece, position))
    }

    pub fn in_directory(path: &str, lua: &mlua::Lua) -> HashMap<String, Self> {
        let mut piece_configs = HashMap::default();

        for piece_config_entry in std::fs::read_dir(path).unwrap() {
            let file_name = piece_config_entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .into_owned();

            let piece_id = file_name.split('.').next().unwrap();
            let piece_config_path = format!("{path}/{piece_id}");

            piece_configs.insert(
                piece_id.to_owned(),
                PieceConfig::from_path(lua, &piece_config_path),
            );
        }

        piece_configs
    }
}
