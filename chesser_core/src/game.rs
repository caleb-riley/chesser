use crate::{board::Board, moves::Move, piece::PieceConfig, position::Position};

use std::collections::HashMap;

pub struct Game {
    pub pieces: HashMap<String, PieceConfig>,
    pub lua: mlua::Lua,
    pub board: Board,
}

impl Game {
    fn load_piece_config(&mut self, id: &str) {
        self.pieces
            .insert(id.to_owned(), PieceConfig::from_script(&self.lua, id));
    }

    pub fn load_piece_configs(&mut self, path: &str) {
        for file in std::fs::read_dir(path).unwrap() {
            let file_name = file.unwrap().file_name().into_string().unwrap();
            let id = file_name.split('.').next().unwrap();

            self.load_piece_config(id);
        }
    }

    fn get_piece_config(&self, id: &str) -> Option<&PieceConfig> {
        self.pieces.get(id)
    }

    pub fn get_piece_value(&self, id: &str) -> i32 {
        self.get_piece_config(id).unwrap().get_value()
    }

    pub fn get_available_moves(&self, id: &str, position: Position) -> Vec<Move> {
        self.get_piece_config(id)
            .unwrap()
            .get_available_moves(&self.lua, &self.board, position)
    }

    pub fn register_helpers(&self) -> mlua::Result<()> {
        let builtins_script = std::fs::read_to_string("./lua/builtins.lua")?;
        self.lua.load(&builtins_script).exec()?;

        let utils_script = std::fs::read_to_string("./lua/utils.lua")?;
        self.lua.load(&utils_script).exec()?;

        let utils: mlua::Table = self.lua.globals().get("utils")?;

        let concat_tables =
            self.lua
                .create_function(|lua, (left, right): (mlua::Table, mlua::Table)| {
                    let new_table = lua.create_table()?;

                    for value in left.sequence_values::<mlua::Value>() {
                        new_table.push(value?)?;
                    }

                    for value in right.sequence_values::<mlua::Value>() {
                        new_table.push(value?)?;
                    }

                    Ok(new_table)
                })?;

        utils.set("concat_tables", concat_tables)?;

        Ok(())
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            pieces: HashMap::new(),
            board: Board::standard(),
            lua: mlua::Lua::new(),
        }
    }
}
