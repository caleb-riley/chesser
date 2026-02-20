use chesser_core::engine::board::Board;
use serde::{Deserialize, Serialize};

use crate::transfer::{PieceDto, PositionDto};

#[derive(Serialize, Deserialize)]
pub struct BoardDto {
    pub dimensions: usize,
    pub pieces: Vec<Vec<Option<PieceDto>>>,
    pub turn_count: usize,
}

impl BoardDto {
    pub fn get_piece(&self, position: PositionDto) -> Option<&PieceDto> {
        self.pieces[position.row][position.column].as_ref()
    }
}

impl From<&Board> for BoardDto {
    fn from(board: &Board) -> Self {
        Self {
            dimensions: board.dimensions(),
            pieces: board
                .squares()
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|id| {
                            id.as_ref()
                                .and_then(|id| board.get_piece_by_id(id).map(PieceDto::from))
                        })
                        .collect()
                })
                .collect(),
            turn_count: board.turn_count(),
        }
    }
}
