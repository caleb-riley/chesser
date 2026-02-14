use std::collections::HashMap;

use mlua::{IntoLua, UserData};

use crate::{
    moves::{Move, MoveKind},
    piece::{Piece, PieceColor},
    position::{Offset, Position},
    utils,
};

fn standard_layout() -> Vec<Vec<Option<Piece>>> {
    let specials: [&'static str; 8] = [
        "rook", "knight", "bishop", "king", "queen", "bishop", "knight", "rook",
    ];

    let mut pieces: Vec<Vec<Option<Piece>>> = (0..8).map(|_| vec![]).collect();

    for kind in specials.iter() {
        pieces[0].push(Some(Piece::new(kind.to_string(), PieceColor::Black)));
    }

    for _ in 0..8 {
        pieces[1].push(Some(Piece::new("pawn".to_owned(), PieceColor::Black)));
    }

    for board_row in pieces.iter_mut().skip(2).take(4) {
        for _ in 0..8 {
            board_row.push(None);
        }
    }

    for _ in 0..8 {
        pieces[6].push(Some(Piece::new("pawn".to_owned(), PieceColor::White)));
    }

    for kind in specials.iter() {
        pieces[7].push(Some(Piece::new(kind.to_string(), PieceColor::White)));
    }

    pieces
}

#[derive(Clone)]
pub struct Board {
    pub dimensions: usize,
    pub turn_count: usize,
    pub captures: HashMap<PieceColor, Vec<String>>,
    pub pieces: Vec<Vec<Option<Piece>>>,
}

impl Board {
    pub fn standard() -> Self {
        Self {
            dimensions: 8,
            turn_count: 0,
            captures: HashMap::from_iter([
                (PieceColor::White, vec![]),
                (PieceColor::Black, vec![]),
            ]),
            pieces: standard_layout(),
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
            piece.kind,
            position.as_notation(),
            the_move.destination.as_notation()
        );

        piece.history.push(the_move.clone());
        piece.last_moved = Some(self.turn_count);

        self.pieces[the_move.destination.row()][the_move.destination.column()] = Some(piece);
    }
}

impl UserData for Board {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get_piece", |lua, board, position| {
            Ok(board.get_piece(position).into_lua(lua))
        });

        methods.add_method("get_perpendicular_moves", |_, board, position| {
            let piece = board.get_piece(position).unwrap();
            let positions = utils::available_positions_in_directions(
                position,
                piece,
                board,
                &utils::PERPENDICULAR,
            );

            Ok(utils::generate_moves(positions, piece, board))
        });

        methods.add_method("get_diagonal_moves", |_, board, position| {
            let piece = board.get_piece(position).unwrap();
            let positions =
                utils::available_positions_in_directions(position, piece, board, &utils::DIAGONAL);

            Ok(utils::generate_moves(positions, piece, board))
        });

        methods.add_method(
            "get_directional_moves",
            |_, board, (position, offsets): (Position, Vec<Offset>)| {
                let piece = board.get_piece(position).unwrap();
                let offsets = offsets
                    .into_iter()
                    .map(|offset| offset.as_parts())
                    .collect::<Vec<_>>();

                let positions =
                    utils::available_positions_in_directions(position, piece, board, &offsets);

                Ok(utils::generate_moves(positions, piece, board))
            },
        );

        methods.add_method("get_dimensions", |_, board, ()| Ok(board.dimensions));

        methods.add_method("in_bounds", |_, board, position: Position| {
            Ok(position.restrict_to(board.dimensions).is_some())
        });

        methods.add_method("turn_count", |_, board, ()| Ok(board.turn_count));
    }
}
