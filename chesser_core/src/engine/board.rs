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
    pub squares: Vec<Vec<Option<String>>>,
    pub pieces: HashMap<String, Piece>,
}

impl Board {
    pub fn from_initial_layout(initial_layout: &mlua::Table, lua: &mlua::Lua) -> Self {
        let dimensions = initial_layout.len().unwrap() as usize;
        let mut squares: Vec<_> = (0..dimensions).map(|_| vec![None; dimensions]).collect();
        let mut pieces = HashMap::default();

        for (row, row_list) in initial_layout.sequence_values::<mlua::Table>().enumerate() {
            for (column, piece) in row_list
                .unwrap()
                .sequence_values::<mlua::Table>()
                .enumerate()
            {
                let piece_info = piece.unwrap();

                if piece_info.is_empty() {
                    continue;
                }

                let id: String = piece_info.get("id").unwrap();
                let color: PieceColor = piece_info.get::<String>("color").unwrap().parse().unwrap();

                let piece = Piece::new(id, color, lua);

                squares[row][column] = Some(piece.id.to_string());
                pieces.insert(piece.id.to_string(), piece);
            }
        }

        Self {
            dimensions,
            turn_count: 0,
            captures: HashMap::from_iter([
                (PieceColor::White, vec![]),
                (PieceColor::Black, vec![]),
            ]),
            squares,
            pieces,
        }
    }

    pub fn get_piece_by_id(&self, id: &str) -> Option<&Piece> {
        self.pieces.get(id)
    }

    pub fn get_piece_at_position(&self, position: Position) -> Option<&Piece> {
        let (row, col) = position.as_parts();

        if row >= self.dimensions || col >= self.dimensions {
            None
        } else {
            let id = self.squares[row][col].as_ref();

            id.map(|id| self.pieces.get(id)).flatten()
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
                    let id = self.squares[origin.row()][origin.column()]
                        .take()
                        .expect("no piece at origin");
                    let piece = self.pieces.get_mut(&id).unwrap();

                    piece.history.push(the_move.clone());
                    piece.last_moved = Some(self.turn_count);

                    self.squares[destination.row()][destination.column()] =
                        Some(piece.id.to_string());
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

                    let piece = Piece::new(id.clone(), *color, lua);

                    self.squares[position.row()][position.column()] = Some(piece.id.to_string());
                    self.pieces.insert(piece.id.to_string(), piece);
                }
                Action::Deletion { position } => {
                    self.squares[position.row()][position.column()] = None;
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
        self.get_matching_positions(|p| self.get_piece_at_position(p).is_none())
    }

    pub fn get_owned_positions(&self, color: PieceColor) -> impl Iterator<Item = Position> {
        self.get_matching_positions(move |p| {
            let Some(piece) = self.get_piece_at_position(p) else {
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
        let squares = (0..8).map(|_| vec![None; 8]).collect();

        Self {
            dimensions: 8,
            turn_count: 0,
            captures: HashMap::from_iter([
                (PieceColor::White, vec![]),
                (PieceColor::Black, vec![]),
            ]),
            squares,
            pieces: HashMap::default(),
        }
    }
}

impl mlua::UserData for Board {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get_piece_by_id", |lua, board, id: String| {
            Ok(board.get_piece_by_id(&id).into_lua(lua))
        });

        methods.add_method("get_piece_at_position", |lua, board, position| {
            Ok(board.get_piece_at_position(position).into_lua(lua))
        });

        methods.add_method("get_perpendicular_moves", |_, board, position| {
            let piece = board.get_piece_at_position(position).unwrap();
            let positions = utils::available_positions_in_directions(
                position,
                piece,
                board,
                &utils::PERPENDICULAR,
            );

            Ok(utils::generate_moves(positions, position, piece, board))
        });

        methods.add_method("get_diagonal_moves", |_, board, position| {
            let piece = board.get_piece_at_position(position).unwrap();
            let positions =
                utils::available_positions_in_directions(position, piece, board, &utils::DIAGONAL);

            Ok(utils::generate_moves(positions, position, piece, board))
        });

        methods.add_method(
            "get_directional_moves",
            |_, board, (position, offsets): (Position, Vec<Offset>)| {
                let piece = board.get_piece_at_position(position).unwrap();
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
