use mlua::{FromLua, IntoLua};

use std::collections::HashMap;

use crate::{
    engine::{board::Board, color::PieceColor, moves::Move, position::Position},
    loader::{GameConfig, PieceConfig},
};

#[derive(Debug)]
pub enum TerminationState {
    Winner(PieceColor),
    Draw,
}

impl IntoLua for TerminationState {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        match self {
            Self::Winner(color) => {
                let s = color.to_string();

                Ok(mlua::Value::String(lua.create_string(&s)?))
            }
            Self::Draw => Ok(mlua::Value::String(lua.create_string("Draw")?)),
        }
    }
}

impl FromLua for TerminationState {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::String(string) = value else {
            return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "string".to_owned(),
                message: Some("TerminationState should be a string".to_owned()),
            });
        };

        match string.to_string_lossy().as_str() {
            "white" => Ok(Self::Winner(PieceColor::White)),
            "black" => Ok(Self::Winner(PieceColor::Black)),
            "draw" => Ok(Self::Draw),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "string",
                to: "PieceColor | draw".to_owned(),
                message: Some("TerminationState should be a string".to_owned()),
            }),
        }
    }
}

pub struct Game {
    piece_configs: HashMap<String, PieceConfig>,
    game_config: GameConfig,
    pub lua: mlua::Lua,
    pub board: Board,
}

impl Game {
    pub fn get_piece_configs(&self) -> &HashMap<String, PieceConfig> {
        &self.piece_configs
    }

    pub fn get_config(&self) -> &GameConfig {
        &self.game_config
    }

    pub fn get_available_moves(
        &self,
        piece_id: &str,
        position: Position,
    ) -> mlua::Result<Vec<Move>> {
        let piece_config = self.piece_configs.get(piece_id).unwrap();

        piece_config.get_available_moves(&self.lua, &self.board, position)
    }

    fn register_utils(lua: &mlua::Lua) -> mlua::Result<()> {
        let builtins_script = std::fs::read_to_string("./lua/builtins.lua")?;
        lua.load(&builtins_script).exec()?;

        let utils_script = std::fs::read_to_string("./lua/utils.lua")?;
        lua.load(&utils_script).exec()?;

        let utils: mlua::Table = lua.globals().get("utils")?;

        let concat_tables =
            lua.create_function(|lua, (left, right): (mlua::Table, mlua::Table)| {
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
        let lua = mlua::Lua::new();
        Self::register_utils(&lua).unwrap();

        let game_config = GameConfig::from_path("./lua/config.lua", &lua);

        let mut board = Board::default();
        board.set_initial_layout(game_config.get_initial_layout(), &lua);

        Self {
            piece_configs: PieceConfig::in_directory("./lua/pieces", &lua),
            game_config,
            board,
            lua,
        }
    }
}
