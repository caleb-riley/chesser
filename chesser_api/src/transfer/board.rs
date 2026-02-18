use chesser_core::engine::{board::Board, color::PieceColor};
use serde::{Deserialize, Serialize};

use crate::transfer::{PieceDto, PositionDto};

#[derive(Serialize, Deserialize)]
pub struct BoardDto {
    pub dimensions: usize,
    pub pieces: Vec<Vec<Option<PieceDto>>>,
    pub current_turn: String,
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
            dimensions: board.dimensions,
            pieces: board
                .squares
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|id| {
                            id.as_ref()
                                .map(|id| board.get_piece_by_id(&id).map(PieceDto::from))
                                .flatten()
                        })
                        .collect()
                })
                .collect(),
            current_turn: PieceColor::from_turn_count(board.turn_count).to_string(),
            turn_count: board.turn_count,
        }
    }
}
