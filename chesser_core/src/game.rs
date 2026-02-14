use crate::{
    board::Board,
    moves::{Move, MoveKind},
    position::{Offset, Position},
};

use std::collections::HashMap;

pub struct PieceConfig {
    value: i32,
    available_moves: mlua::Function,
}

impl PieceConfig {
    fn from_script(lua: &mlua::Lua, id: &str) -> Self {
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

        self.available_moves
            .call((board_userdata, position))
            .unwrap()
    }
}

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
        // let cloned_self = Arc::clone(&self); // clone Arc, no lifetime issues

        // let get_perpendicular_moves =
        //     self.lua.create_function(move |lua, position: Position| {
        //         let piece = self.board.get_piece(position).unwrap();
        //         let positions = utils::available_positions_in_directions(
        //             piece,
        //             &self.board,
        //             &utils::PERPENDICULAR,
        //         );
        //         let moves = utils::generate_moves(positions, piece, &self.board);

        //         Ok(vec_into_lua(lua, moves.into_iter().collect()))
        //     })?;

        let utils = self.lua.create_table()?;
        // utils.set("perpendicular_moves", get_perpendicular_moves)?;

        // local function make_passive_move(destination)
        //     return { destination = destination, kind = "passive" }
        // end

        let make_position = self
            .lua
            .create_function(move |_, (row, column)| Ok(Position::new(row, column)))?;

        let make_offset = self
            .lua
            .create_function(move |_, (delta_row, delta_column)| {
                Ok(Offset::new(delta_row, delta_column))
            })?;

        let make_passive_move = self
            .lua
            .create_function(|_, destination| Ok(Move::new(destination, MoveKind::Passive)))?;

        let make_capture_move = self.lua.create_function(|_, (destination, captures)| {
            Ok(Move::new(destination, MoveKind::Capture(captures)))
        })?;

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

        utils.set("make_position", make_position)?;
        utils.set("make_offset", make_offset)?;

        utils.set("make_passive_move", make_passive_move)?;
        utils.set("make_capture_move", make_capture_move)?;

        utils.set("concat_tables", concat_tables)?;

        self.lua.globals().set("utils", utils)?;

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
