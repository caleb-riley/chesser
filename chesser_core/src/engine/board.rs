use mlua::IntoLua;
use std::{collections::HashMap, io::Write};

use crate::engine::{
    action::Action,
    color::PieceColor,
    game::hook_names,
    moves::Move,
    piece::Piece,
    position::{Offset, Position},
    utils,
};

#[derive(Clone)]
pub struct Board {
    dimensions: usize,
    turn_count: usize,
    captures: HashMap<PieceColor, Vec<String>>,
    squares: Vec<Vec<Option<String>>>,
    pieces: HashMap<String, Piece>,
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

    pub fn dimensions(&self) -> usize {
        self.dimensions
    }

    pub fn turn_count(&self) -> usize {
        self.turn_count
    }

    pub fn captures(&self) -> &HashMap<PieceColor, Vec<String>> {
        &self.captures
    }

    pub fn squares(&self) -> &Vec<Vec<Option<String>>> {
        &self.squares
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

            id.and_then(|id| self.pieces.get(id))
        }
    }

    pub fn perform_move(
        &mut self,
        mv: &Move,
        lua: &mlua::Lua,
        hooks: &HashMap<String, mlua::Function>,
    ) {
        for action in mv.actions() {
            match action {
                Action::Relocate {
                    origin,
                    destination,
                } => {
                    let id = self.squares[origin.row()][origin.column()]
                        .take()
                        .expect("no piece at origin");
                    let piece = self.pieces.get_mut(&id).unwrap();

                    piece.history.push(mv.clone());
                    piece.last_moved = Some(self.turn_count);

                    self.squares[destination.row()][destination.column()] =
                        Some(piece.id.to_string());

                    if let Some(hook) = hooks.get(hook_names::ON_PIECE_RELOCATED) {
                        let _: mlua::Value = hook.call(piece.id.to_string()).unwrap();
                    }
                }

                Action::Spawn {
                    position,
                    kind: id,
                    color,
                } => {
                    let piece = Piece::new(id.clone(), *color, lua);
                    let piece_id = piece.id.to_string();

                    self.squares[position.row()][position.column()] = Some(piece.id.to_string());
                    self.pieces.insert(piece.id.to_string(), piece);

                    if let Some(hook) = hooks.get(hook_names::ON_PIECE_SPAWNED) {
                        let _: mlua::Value = hook.call(piece_id.to_string()).unwrap();
                    }
                }
                Action::Deletion { position } => {
                    if let Some(captured) = self.get_piece_at_position(*position)
                        && let Some(hook) = hooks.get(hook_names::ON_PIECE_DELETED)
                    {
                        let _: mlua::Value = hook.call(captured.id.to_string()).unwrap();
                    }

                    self.squares[position.row()][position.column()] = None;
                }
            }
        }

        if !mv.promotions().is_empty() {
            let chosen_kind = loop {
                print!("Enter promotion ({}): ", mv.promotions().join(", "));

                let mut buffer = String::new();
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut buffer).unwrap();

                let chosen_kind = buffer.trim().to_string();

                if mv.promotions().contains(&chosen_kind) {
                    break chosen_kind;
                }

                println!("Invalid choice, please try again");
            };

            let current_piece = self.get_piece_at_position(mv.destination()).unwrap();
            let new_piece = Piece::new(chosen_kind.clone(), current_piece.color, lua);

            self.squares[mv.destination().row()][mv.destination().column()] =
                Some(new_piece.id.to_string());
            self.pieces.insert(new_piece.id.to_string(), new_piece);
        }

        if let Some(hook) = hooks.get(hook_names::ON_TURN_ENDED) {
            let _: mlua::Value = hook
                .call(PieceColor::from_turn_count(self.turn_count))
                .unwrap();
        }

        self.turn_count += 1;

        if let Some(hook) = hooks.get(hook_names::ON_TURN_STARTED) {
            let _: mlua::Value = hook
                .call(PieceColor::from_turn_count(self.turn_count))
                .unwrap();
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
