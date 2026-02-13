use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Position(usize, usize);

impl Position {
    pub fn new(row: usize, column: usize) -> Self {
        Self(row, column)
    }

    pub fn row(&self) -> usize {
        self.0
    }

    pub fn column(&self) -> usize {
        self.1
    }

    pub fn as_parts(&self) -> (usize, usize) {
        (self.0, self.1)
    }

    pub fn offset_by(&self, offset: Offset) -> Self {
        Self(
            (self.0 as isize + offset.delta_row()) as usize,
            (self.1 as isize + offset.delta_column()) as usize,
        )
    }

    pub fn restrict_to(&self, size: usize) -> Option<Self> {
        if self.0 >= size || self.1 >= size {
            return None;
        }

        Some(*self)
    }

    pub fn offset_to(&self, other: Self) -> Offset {
        Offset::new(
            other.row() as isize - self.row() as isize,
            other.column() as isize - self.column() as isize,
        )
    }

    pub fn as_notation(&self) -> String {
        format!(
            "{}{}",
            (b'a' + self.1 as u8) as char,
            (b'8' - self.0 as u8) as char
        )
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl mlua::IntoLua for Position {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table_from([("row", self.row()), ("column", self.column())])?;

        Ok(mlua::Value::Table(table))
    }
}

impl mlua::FromLua for Position {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::Table(table) = value else {
            return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Position".to_string(),
                message: Some("expected table with fields 'row' and 'col'".into()),
            });
        };

        let row: usize = table.get("row")?;
        let col: usize = table.get("column")?;

        Ok(Position::new(row, col))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Offset(isize, isize);

impl Offset {
    pub fn new(delta_row: isize, delta_column: isize) -> Self {
        Self(delta_row, delta_column)
    }

    pub fn delta_row(&self) -> isize {
        self.0
    }

    pub fn delta_column(&self) -> isize {
        self.1
    }

    pub fn scale_by(&self, factor: isize) -> Self {
        Self(self.0 * factor, self.1 * factor)
    }

    pub fn taxicab_magnitude(&self) -> usize {
        self.0.unsigned_abs() + self.1.unsigned_abs()
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
