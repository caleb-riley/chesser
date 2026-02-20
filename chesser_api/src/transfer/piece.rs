use chesser_core::{engine::piece::Piece, loader::PieceConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct PieceConfigDto {
    pub value: i32,
}

impl From<&PieceConfig> for PieceConfigDto {
    fn from(piece_config: &PieceConfig) -> Self {
        Self {
            value: piece_config.piece_value(),
        }
    }
}
