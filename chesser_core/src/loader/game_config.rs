use crate::engine::{board::Board, game::TerminationState};

pub struct GameConfig {
    check_termination: mlua::Function,
}

impl GameConfig {
    const TERMINATION_FUNCTION: &str = "check_termination";

    pub fn from_path(script_path: &str, lua: &mlua::Lua) -> Self {
        let script = std::fs::read_to_string(script_path).unwrap();
        let game_config: mlua::Table = lua.load(script).eval().unwrap();

        Self {
            check_termination: game_config.get(Self::TERMINATION_FUNCTION).unwrap(),
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
}
