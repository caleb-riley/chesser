use mlua::IntoLua;
use std::{collections::HashMap, io::Write};

use crate::engine::{
    action::Action,
    color::PieceColor,
    moves::Move,
    piece::Piece,
    position::{Offset, Position},
    utils,
};

#[derive(Clone)]
pub struct Board {
    pub dimensions: usize,
    pub turn_count: usize,
    pub captures: HashMap<PieceColor, Vec<String>>,
    pub pieces: Vec<Vec<Option<Piece>>>,
}

impl Board {
    pub fn set_initial_layout(&mut self, initial_layout: &mlua::Table, lua: &mlua::Lua) {
        for (row, row_list) in initial_layout.sequence_values::<mlua::Table>().enumerate() {
            for (column, piece) in row_list
                .unwrap()
                .sequence_values::<mlua::Table>()
                .enumerate()
            {
                let piece = piece.unwrap();

                if piece.is_empty() {
                    continue;
                }

                let piece_id: String = piece.get("id").unwrap();
                let color: PieceColor = piece.get::<String>("color").unwrap().parse().unwrap();

                self.pieces[row][column] = Some(Piece::new(piece_id, color, lua))
            }
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

    pub fn perform_move(&mut self, the_move: &Move, lua: &mlua::Lua) {
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
                        Some(Piece::new(id.clone(), *color, lua));
                }
                Action::Deletion { position } => {
                    self.pieces[position.row()][position.column()] = None;
                }
            }
        }
    }

    fn get_matching_positions<F>(&self, filter: F) -> impl Iterator<Item = Position>
    where
        F: Fn(Position) -> bool,
    {
        let dims = self.dimensions;

        (0..dims * dims).filter_map(move |i| {
            let row = i / dims;
            let column = i % dims;

            let pos = Position::new(row, column);
            filter(pos).then_some(pos)
        })
    }

    pub fn get_empty_positions(&self) -> impl Iterator<Item = Position> {
        self.get_matching_positions(|p| self.get_piece(p).is_none())
    }

    pub fn get_owned_positions(&self, color: PieceColor) -> impl Iterator<Item = Position> {
        self.get_matching_positions(move |p| {
            let Some(piece) = self.get_piece(p) else {
                return false;
            };

            piece.color == color
        })
    }

    pub fn get_area_positions(&self, ul: Position, br: Position) -> impl Iterator<Item = Position> {
        self.get_matching_positions(move |p| {
            (ul.row()..=br.row()).contains(&p.row())
                && (ul.column()..br.column()).contains(&p.column())
        })
    }
}

impl Default for Board {
    fn default() -> Self {
        let pieces = (0..8).map(|_| vec![None; 8]).collect();

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
