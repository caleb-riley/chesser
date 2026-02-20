use std::collections::HashMap;

use crate::{
    engine::{board::Board, moves::Move, position::Position},
    loader::load_table,
};

pub struct PieceConfig {
    piece_value: i32,
    available_moves: mlua::Function,
}

impl PieceConfig {
    pub fn from_path(lua: &mlua::Lua, piece_path: &str) -> Self {
        let script_path = format!("{piece_path}/piece.lua");
        let piece_config = load_table(&script_path, lua).unwrap();

        let value: mlua::Integer = piece_config.get("value").unwrap();
        let available_moves: mlua::Function = piece_config.get("available_moves").unwrap();

        Self {
            piece_value: value as i32,
            available_moves,
        }
    }

    pub fn piece_value(&self) -> i32 {
        self.piece_value
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
