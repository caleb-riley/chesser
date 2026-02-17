use mlua::IntoLua;

use crate::engine::{action::Action, position::Position};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Move {
    pub origin: Position,
    pub destination: Position,
    pub actions: Vec<Action>,
    pub promotions: Vec<String>,
}

impl Move {
    pub fn new(
        origin: Position,
        destination: Position,
        actions: Vec<Action>,
        promotions: Vec<String>,
    ) -> Self {
        Self {
            origin,
            destination,
            actions,
            promotions,
        }
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

        let origin_table: mlua::Table = table.get("origin")?;
        let origin = Position::from_lua(mlua::Value::Table(origin_table), _lua)?;

        let dest_table: mlua::Table = table.get("destination")?;
        let destination = Position::from_lua(mlua::Value::Table(dest_table), _lua)?;

        let actions_table: mlua::Table = table.get("actions")?;
        let mut actions = vec![];

        for action in actions_table.sequence_values::<Action>() {
            actions.push(action?);
        }

        let promotions_table: mlua::Table = table.get("promotions")?;
        let mut promotions = vec![];

        for promotion in promotions_table.sequence_values::<String>() {
            promotions.push(promotion?);
        }

        Ok(Move::new(origin, destination, actions, promotions))
    }
}

impl IntoLua for Move {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;

        table.set("origin", self.origin.into_lua(lua)?)?;
        table.set("destination", self.destination.into_lua(lua)?)?;
        table.set("actions", self.actions.into_lua(lua)?)?;
        table.set("promotions", self.promotions.into_lua(lua)?)?;

        Ok(mlua::Value::Table(table))
    }
}
