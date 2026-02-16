use chesser_core::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
pub struct PositionDto {
    pub row: usize,
    pub column: usize,
}

impl From<&Position> for PositionDto {
    fn from(position: &Position) -> Self {
        Self {
            row: position.row(),
            column: position.column(),
        }
    }
}

impl From<PositionDto> for Position {
    fn from(position: PositionDto) -> Self {
        Self::new(position.row, position.column)
    }
}
