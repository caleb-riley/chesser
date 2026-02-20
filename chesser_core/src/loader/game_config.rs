use std::collections::HashMap;

use crate::{
    engine::{board::Board, game::TerminationState},
    loader::load_table,
};

pub struct GameConfig {
    check_termination: mlua::Function,
    initial_layout: mlua::Table,
    hooks: HashMap<String, mlua::Function>,
}

impl GameConfig {
    const TERMINATION_FUNCTION: &str = "check_termination";
    const INITIAL_LAYOUT_KEY: &str = "initial_layout";

    pub fn from_mod_root(mod_path: &str, lua: &mlua::Lua) -> Self {
        let game_config_path = format!("{mod_path}/config.lua");
        let game_config = load_table(&game_config_path, lua).unwrap();

        let hooks_path = format!("{mod_path}/hooks.lua");
        let hooks_table = load_table(&hooks_path, lua).unwrap();

        Self {
            check_termination: game_config.get(Self::TERMINATION_FUNCTION).unwrap(),
            initial_layout: game_config.get(Self::INITIAL_LAYOUT_KEY).unwrap(),
            hooks: hooks_table.pairs().map(Result::unwrap).collect(),
        }
    }

    pub fn check_termination(
        &self,
        board: &Board,
        lua: &mlua::Lua,
    ) -> mlua::Result<Option<TerminationState>> {
        let board_userdata = lua.create_userdata(board.clone()).unwrap();

        self.check_termination.call(board_userdata)
    }

    pub fn get_hooks(&self) -> &HashMap<String, mlua::Function> {
        &self.hooks
    }

    pub fn get_initial_layout(&self) -> &mlua::Table {
        &self.initial_layout
    }
}
