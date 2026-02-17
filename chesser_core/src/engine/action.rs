use crate::engine::{color::PieceColor, position::Position};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Action {
    Relocate {
        origin: Position,
        destination: Position,
    },
    Spawn {
        position: Position,
        id: String,
        color: PieceColor,
    },
    Deletion {
        position: Position,
    },
}

impl mlua::FromLua for Action {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::Table(table) = value else {
            return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Move".to_string(),
                message: Some("expected table".into()),
            });
        };

        let kind = table.get::<String>("kind")?;

        match kind.as_str() {
            "relocation" => {
                let origin: Position = table.get("origin")?;
                let destination: Position = table.get("destination")?;

                Ok(Action::Relocate {
                    origin,
                    destination,
                })
            }
            "spawn" => {
                let position: Position = table.get("position")?;
                let id: String = table.get("id")?;
                let color: PieceColor = table.get("color")?;

                Ok(Action::Spawn {
                    position,
                    id,
                    color,
                })
            }
            "deletion" => {
                let position: Position = table.get("position")?;

                Ok(Action::Deletion { position })
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "table",
                to: "Action".to_string(),
                message: Some("expected table".into()),
            }),
        }
    }
}

impl mlua::IntoLua for Action {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;

        match self {
            Self::Relocate {
                origin,
                destination,
            } => {
                table.set("kind", "relocation")?;
                table.set("origin", origin)?;
                table.set("destination", destination)?;
            }
            Self::Spawn {
                position,
                id,
                color,
            } => {
                table.set("kind", "spawn")?;
                table.set("position", position)?;
                table.set("id", id)?;
                table.set("color", color)?;
            }
            Self::Deletion { position } => {
                table.set("kind", "deletion")?;
                table.set("position", position)?;
            }
        }

        Ok(mlua::Value::Table(table))
    }
}
