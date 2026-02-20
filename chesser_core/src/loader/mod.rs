mod game_config;
mod piece_config;

pub use game_config::GameConfig;
pub use piece_config::PieceConfig;

fn load_table(script_path: &str, lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let script = std::fs::read_to_string(script_path).unwrap();
    lua.load(script).eval()
}
