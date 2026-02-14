use chesser_core::{board::Board, piece::PieceColor};
use serde::Serialize;

use crate::transfer::piece::PieceDto;

#[derive(Serialize)]
pub struct BoardDto {
    pub pieces: Vec<Vec<Option<PieceDto>>>,
    pub turn: String,
}

impl From<&Board> for BoardDto {
    fn from(board: &Board) -> Self {
        Self {
            pieces: board
                .pieces
                .iter()
                .map(|row| row.iter().map(|p| p.as_ref().map(PieceDto::from)).collect())
                .collect(),
            turn: PieceColor::from_turn_count(board.turn_count).to_string(),
        }
    }
}
