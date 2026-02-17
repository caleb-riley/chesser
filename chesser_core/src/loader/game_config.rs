use crate::engine::{board::Board, game::TerminationState};

pub struct GameConfig {
    check_termination: mlua::Function,
    initial_layout: mlua::Table,
}

impl GameConfig {
    const TERMINATION_FUNCTION: &str = "check_termination";
    const INITIAL_LAYOUT_KEY: &str = "initial_layout";

    pub fn from_path(script_path: &str, lua: &mlua::Lua) -> Self {
        let script = std::fs::read_to_string(script_path).unwrap();
        let game_config: mlua::Table = lua.load(script).eval().unwrap();

        Self {
            check_termination: game_config.get(Self::TERMINATION_FUNCTION).unwrap(),
            initial_layout: game_config.get(Self::INITIAL_LAYOUT_KEY).unwrap(),
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

    pub fn get_initial_layout(&self) -> &mlua::Table {
        &self.initial_layout
    }
}
