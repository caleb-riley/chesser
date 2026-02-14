use chesser_core::piece::Piece;
use serde::Serialize;

#[derive(Serialize)]
pub struct PieceDto {
    pub kind: String,
    pub color: String,
}

impl From<&Piece> for PieceDto {
    fn from(piece: &Piece) -> Self {
        Self {
            kind: piece.kind.to_string(),
            color: piece.color.to_string(),
        }
    }
}
