use std::collections::{HashMap, HashSet};

use crate::{
    moves::{Move, MoveKind},
    piece::{Piece, PieceColor, PieceKind},
    position::{Offset, Position},
};

#[derive(Debug)]
pub struct Board {
    pub dimensions: usize,
    pub turn_count: usize,
    pub captures: HashMap<PieceColor, Vec<PieceKind>>,
    pieces: Vec<Vec<Option<Piece>>>,
}

impl Board {
    const PERPENDICULAR: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    const DIAGONAL: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];
    const STAGGERED: [(isize, isize); 8] = [
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
    ];

    pub fn standard() -> Self {
        const SPECIALS: [PieceKind; 8] = [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::King,
            PieceKind::Queen,
            PieceKind::Bishop,
            PieceKind::Knight,
            PieceKind::Rook,
        ];

        let mut pieces: Vec<Vec<Option<Piece>>> = (0..8).map(|_| vec![]).collect();

        for (index, kind) in SPECIALS.iter().enumerate() {
            pieces[0].push(Some(Piece::new(
                *kind,
                PieceColor::Black,
                Position::new(0, index),
            )));
        }

        for index in 0..8 {
            pieces[1].push(Some(Piece::new(
                PieceKind::Pawn,
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
                PieceKind::Pawn,
                PieceColor::White,
                Position::new(6, index),
            )));
        }

        for (index, kind) in SPECIALS.iter().enumerate() {
            pieces[7].push(Some(Piece::new(
                *kind,
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

    pub fn perform_move(&mut self, position: Position, the_move: Move) {
        self.turn_count += 1;

        let mut piece = self.pieces[position.row()][position.column()]
            .take()
            .unwrap();

        if let MoveKind::Capture(capture_position) = the_move.kind() {
            let captured = self.pieces[capture_position.row()][capture_position.column()]
                .take()
                .unwrap();

            self.captures
                .get_mut(&piece.color)
                .unwrap()
                .push(captured.kind);
        }

        let destination = the_move.destination();

        piece.position = destination;
        piece.turns += 1;
        piece.last_moved = Some(self.turn_count);

        self.pieces[destination.row()][destination.column()] = Some(piece);
    }

    fn available_moves_in_directions(
        &self,
        piece: &Piece,
        directions: &[(isize, isize)],
    ) -> HashSet<Move> {
        let mut moves = HashSet::new();

        for (delta_row, delta_column) in directions {
            let mut scale = 1;

            while let Some(target_position) = piece
                .position
                .offset_by(Offset::new(*delta_row, *delta_column).scale_by(scale))
                .restrict_to(self.dimensions)
            {
                if let Some(target) = self.get_piece(target_position) {
                    if target.color != piece.color {
                        moves.insert(Move::new(
                            target_position,
                            MoveKind::Capture(target_position),
                        ));
                    }

                    break;
                } else {
                    moves.insert(Move::new(target_position, MoveKind::Passive));
                }

                scale += 1;
            }
        }

        moves
    }

    pub fn available_moves(&self, piece: &Piece) -> HashSet<Move> {
        let mut moves = HashSet::new();

        match piece.kind {
            PieceKind::Pawn => {
                let delta_row = (piece.color == PieceColor::Black) as isize * 2 - 1;

                if let Some(forward_one) = piece
                    .position
                    .offset_by(Offset::new(delta_row, 0))
                    .restrict_to(self.dimensions)
                    && self.get_piece(forward_one).is_none()
                {
                    moves.insert(Move::new(forward_one, MoveKind::Passive));
                }

                if piece.turns == 0
                    && let Some(forward_two) = piece
                        .position
                        .offset_by(Offset::new(delta_row, 0).scale_by(2))
                        .restrict_to(self.dimensions)
                    && self.get_piece(forward_two).is_none()
                {
                    moves.insert(Move::new(forward_two, MoveKind::Passive));
                }

                for delta_column in [-1isize, 1isize] {
                    if let Some(diagonal_position) = piece
                        .position
                        .offset_by(Offset::new(delta_row, delta_column))
                        .restrict_to(self.dimensions)
                    {
                        if let Some(other) = self.get_piece(diagonal_position)
                            && other.color != piece.color
                        {
                            moves.insert(Move::new(
                                diagonal_position,
                                MoveKind::Capture(diagonal_position),
                            ));
                        }

                        let lateral_position = piece.position.offset_by(Offset::new(0, delta_row));

                        if let Some(other) = self.get_piece(lateral_position)
                            && other.color != piece.color
                            && other.previous.offset_to(other.position).taxicab_magnitude() == 2
                            && other.last_moved == Some(self.turn_count)
                        {
                            moves.insert(Move::new(
                                diagonal_position,
                                MoveKind::Capture(lateral_position),
                            ));
                        }
                    }
                }
            }
            PieceKind::Knight => {
                for (delta_row, delta_column) in Self::STAGGERED {
                    let Some(target_position) = piece
                        .position
                        .offset_by(Offset::new(delta_row, delta_column))
                        .restrict_to(self.dimensions)
                    else {
                        continue;
                    };

                    if let Some(target) = self.get_piece(target_position) {
                        if target.color != piece.color {
                            moves.insert(Move::new(
                                target_position,
                                MoveKind::Capture(target_position),
                            ));
                        }
                    } else {
                        moves.insert(Move::new(target_position, MoveKind::Passive));
                    }
                }
            }
            PieceKind::Bishop => {
                self.available_moves_in_directions(piece, &Self::DIAGONAL)
                    .iter()
                    .for_each(|&m| {
                        moves.insert(m);
                    });
            }
            PieceKind::Rook => {
                self.available_moves_in_directions(piece, &Self::PERPENDICULAR)
                    .iter()
                    .for_each(|&m| {
                        moves.insert(m);
                    });
            }
            PieceKind::King => {
                let neighbor_offsets = Self::PERPENDICULAR
                    .into_iter()
                    .chain(Self::DIAGONAL)
                    .collect::<Vec<_>>();

                for (delta_row, delta_column) in neighbor_offsets {
                    let Some(target_position) = piece
                        .position
                        .offset_by(Offset::new(delta_row, delta_column))
                        .restrict_to(self.dimensions)
                    else {
                        continue;
                    };

                    if let Some(target) = self.get_piece(target_position) {
                        if target.color != piece.color {
                            moves.insert(Move::new(
                                target_position,
                                MoveKind::Capture(target_position),
                            ));
                        }
                    } else {
                        moves.insert(Move::new(target_position, MoveKind::Passive));
                    }
                }
            }
            PieceKind::Queen => {
                self.available_moves_in_directions(piece, &Self::PERPENDICULAR)
                    .iter()
                    .for_each(|&m| {
                        moves.insert(m);
                    });

                self.available_moves_in_directions(piece, &Self::DIAGONAL)
                    .iter()
                    .for_each(|&m| {
                        moves.insert(m);
                    });
            }
        }

        moves
    }
}
