use mlua::IntoLua;

use crate::position::Position;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum MoveKind {
    Passive,
    Capture(Vec<Position>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Move {
    pub destination: Position,
    pub kind: MoveKind,
}

impl Move {
    pub fn new(destination: Position, kind: MoveKind) -> Self {
        Self { destination, kind }
    }
}

impl mlua::FromLua for Move {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        let table = match value {
            mlua::Value::Table(t) => t,
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: value.type_name(),
                    to: "Move".to_string(),
                    message: Some("expected table".into()),
                });
            }
        };

        // -------- destination --------
        let dest_table: mlua::Table = table.get("destination")?;
        let destination = Position::from_lua(mlua::Value::Table(dest_table), _lua)?;

        // -------- kind --------
        let kind_value: mlua::Value = table.get("kind")?;

        let kind = match kind_value {
            mlua::Value::String(s) => {
                let kind_str = s.to_str()?.to_string();

                match kind_str.as_str() {
                    "passive" => MoveKind::Passive,
                    _ => {
                        return Err(mlua::Error::FromLuaConversionError {
                            from: "string",
                            to: "MoveKind".to_string(),
                            message: Some(format!("unknown kind: {}", kind_str)),
                        });
                    }
                }
            }

            mlua::Value::Table(kind_table) => {
                let kind_type: String = kind_table.get("type")?;

                match kind_type.as_str() {
                    "capture" => {
                        let captured_table: mlua::Table = kind_table.get("captures")?;

                        let mut captured_positions = Vec::new();
                        for pos in captured_table.sequence_values::<Position>() {
                            captured_positions.push(pos?);
                        }

                        MoveKind::Capture(captured_positions)
                    }
                    _ => {
                        return Err(mlua::Error::FromLuaConversionError {
                            from: "table",
                            to: "MoveKind".to_string(),
                            message: Some(format!("unknown kind type: {}", kind_type)),
                        });
                    }
                }
            }

            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: kind_value.type_name(),
                    to: "MoveKind".to_string(),
                    message: Some("invalid kind format".into()),
                });
            }
        };

        Ok(Move { destination, kind })
    }
}

impl IntoLua for Move {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        // Create a Lua table for the Move
        let table = lua.create_table()?;

        // destination
        table.set("destination", self.destination.into_lua(lua)?)?;

        // kind
        match self.kind {
            MoveKind::Passive => {
                // just a string for passive
                let s: mlua::String = lua.create_string("passive")?;
                table.set("kind", s)?;
            }
            MoveKind::Capture(captured_positions) => {
                // table with type = "capture" and captured = [...]
                let kind_table = lua.create_table()?;
                kind_table.set("type", "capture")?;

                let captured_table = lua.create_table()?;
                for (i, pos) in captured_positions.into_iter().enumerate() {
                    captured_table.set(i + 1, pos.into_lua(lua)?)?;
                }

                kind_table.set("captures", captured_table)?;
                table.set("kind", kind_table)?;
            }
        }

        Ok(mlua::Value::Table(table))
    }
}
