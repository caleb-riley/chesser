use mlua::IntoLua;
use std::{collections::HashMap, io::Write};

use crate::{
    action::Action,
    moves::Move,
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

    pub fn perform_move(&mut self, the_move: &Move) {
        self.turn_count += 1;

        for action in &the_move.actions {
            match action {
                Action::Relocate {
                    origin,
                    destination,
                } => {
                    let mut piece = self.pieces[origin.row()][origin.column()]
                        .take()
                        .expect("no piece at origin");

                    piece.history.push(the_move.clone());
                    piece.last_moved = Some(self.turn_count);

                    self.pieces[destination.row()][destination.column()] = Some(piece);
                }

                Action::Spawn {
                    position,
                    id,
                    color,
                } => {
                    let mut id = id.clone();

                    if id == "PROMOTION" {
                        let mut buffer = String::new();

                        print!("Enter promotion: ");
                        std::io::stdout().flush().unwrap();

                        std::io::stdin().read_line(&mut buffer).unwrap();

                        id = buffer.trim().to_string();
                    }

                    self.pieces[position.row()][position.column()] =
                        Some(Piece::new(id.clone(), color.clone()));
                }
                Action::Deletion { position } => {
                    self.pieces[position.row()][position.column()] = None;
                }
            }
        }
    }
}

impl mlua::UserData for Board {
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

            Ok(utils::generate_moves(positions, position, piece, board))
        });

        methods.add_method("get_diagonal_moves", |_, board, position| {
            let piece = board.get_piece(position).unwrap();
            let positions =
                utils::available_positions_in_directions(position, piece, board, &utils::DIAGONAL);

            Ok(utils::generate_moves(positions, position, piece, board))
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

                Ok(utils::generate_moves(positions, position, piece, board))
            },
        );

        methods.add_method("get_dimensions", |_, board, ()| Ok(board.dimensions));

        methods.add_method("turn_count", |_, board, ()| Ok(board.turn_count));
    }
}
