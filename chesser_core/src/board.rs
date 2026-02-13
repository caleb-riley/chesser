use std::{collections::HashMap, sync::Arc};

use crate::{
    kind::*,
    moves::{Move, MoveKind},
    piece::{Piece, PieceColor},
    position::Position,
};

pub struct Board {
    pub dimensions: usize,
    pub turn_count: usize,
    pub captures: HashMap<PieceColor, Vec<Arc<dyn PieceKind + Send + Sync + 'static>>>,
    pub pieces: Vec<Vec<Option<Piece>>>,
}

impl Board {
    pub fn standard() -> Self {
        let specials: [Arc<dyn PieceKind>; 8] = [
            Arc::new(Rook),
            Arc::new(Knight),
            Arc::new(Bishop),
            Arc::new(King),
            Arc::new(Queen),
            Arc::new(Bishop),
            Arc::new(Knight),
            Arc::new(Rook),
        ];

        let pawn: Arc<dyn PieceKind> = Arc::new(Pawn);

        let mut pieces: Vec<Vec<Option<Piece>>> = (0..8).map(|_| vec![]).collect();

        for (index, kind) in specials.iter().enumerate() {
            pieces[0].push(Some(Piece::new(
                Arc::clone(kind),
                PieceColor::Black,
                Position::new(0, index),
            )));
        }

        for index in 0..8 {
            pieces[1].push(Some(Piece::new(
                Arc::clone(&pawn),
                PieceColor::Black,
                Position::new(1, index),
            )));
        }

        for board_row in pieces.iter_mut().skip(2).take(4) {
            for _ in 0..8 {
                board_row.push(None);
            }
        }

        for index in 0..8 {
            pieces[6].push(Some(Piece::new(
                Arc::clone(&pawn),
                PieceColor::White,
                Position::new(6, index),
            )));
        }

        for (index, kind) in specials.iter().enumerate() {
            pieces[7].push(Some(Piece::new(
                Arc::clone(kind),
                PieceColor::White,
                Position::new(7, index),
            )));
        }

        Self {
            dimensions: 8,
            turn_count: 0,
            captures: HashMap::from_iter([
                (PieceColor::White, vec![]),
                (PieceColor::Black, vec![]),
            ]),
            pieces,
        }
    }

    pub fn get_piece(&self, position: Position) -> Option<&Piece> {
        let (row, col) = position.as_parts();

        if row >= self.dimensions || col >= self.dimensions {
            None
        } else {
            self.pieces[row][col].as_ref()
        }
    }

    pub fn perform_move(&mut self, position: Position, the_move: &Move) {
        self.turn_count += 1;

        let mut piece = self.pieces[position.row()][position.column()]
            .take()
            .unwrap();

        if let MoveKind::Capture(capture_positions) = &the_move.kind {
            for capture_position in capture_positions {
                let captured = self.pieces[capture_position.row()][capture_position.column()]
                    .take()
                    .unwrap();

                self.captures
                    .get_mut(&piece.color)
                    .unwrap()
                    .push(captured.kind);
            }
        }

        println!(
            "[{}] {} to {}",
            piece.kind.name(),
            piece.position.as_notation(),
            the_move.destination.as_notation()
        );

        piece.history.push(piece.position);
        piece.position = the_move.destination;
        piece.last_moved = Some(self.turn_count);

        self.pieces[the_move.destination.row()][the_move.destination.column()] = Some(piece);
    }
}
