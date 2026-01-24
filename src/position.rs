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
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
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

    // pub fn as_parts(&self) -> (isize, isize) {
    //     (self.0, self.1)
    // }

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
