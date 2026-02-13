use std::collections::HashMap;

use mlua::IntoLua;

use crate::{
    moves::{Move, MoveKind},
    piece::{Piece, PieceColor},
    position::Position,
};

pub struct Board {
    pub dimensions: usize,
    pub turn_count: usize,
    pub captures: HashMap<PieceColor, Vec<String>>,
    pub pieces: Vec<Vec<Option<Piece>>>,
}

fn standard_setup() -> Vec<Vec<Option<Piece>>> {
    let specials: [&'static str; 8] = [
        "rook", "knight", "bishop", "king", "queen", "bishop", "knight", "rook",
    ];

    let mut pieces: Vec<Vec<Option<Piece>>> = (0..8).map(|_| vec![]).collect();

    for (index, kind) in specials.iter().enumerate() {
        pieces[0].push(Some(Piece::new(
            kind.to_string(),
            PieceColor::Black,
            Position::new(0, index),
        )));
    }

    for index in 0..8 {
        pieces[1].push(Some(Piece::new(
            "pawn".to_owned(),
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
            "pawn".to_owned(),
            PieceColor::White,
            Position::new(6, index),
        )));
    }

    for (index, kind) in specials.iter().enumerate() {
        pieces[7].push(Some(Piece::new(
            kind.to_string(),
            PieceColor::White,
            Position::new(7, index),
        )));
    }

    pieces
}

impl Board {
    pub fn standard() -> Self {
        // let mut config = GameConfig::default();

        // config.load_piece_configs("./pieces");

        Self {
            dimensions: 8,
            turn_count: 0,
            captures: HashMap::from_iter([
                (PieceColor::White, vec![]),
                (PieceColor::Black, vec![]),
            ]),
            pieces: standard_setup(),
            // config,
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
            piece.position.as_notation(),
            the_move.destination.as_notation()
        );

        // piece.history.push(piece.position);
        piece.position = the_move.destination;
        piece.last_moved = Some(self.turn_count);

        self.pieces[the_move.destination.row()][the_move.destination.column()] = Some(piece);
    }
}

impl IntoLua for &Board {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let rows = lua.create_table_with_capacity(self.dimensions, 0)?;

        for row_index in 0..self.dimensions {
            let row = lua.create_table_with_capacity(self.dimensions, 0)?;

            for column_index in 0..self.dimensions {
                if let Some(piece) = &self.pieces[row_index][column_index] {
                    row.push(piece.into_lua(lua)?)?;
                } else {
                    row.push(mlua::Value::Nil)?;
                }
            }

            rows.push(row)?;
        }

        Ok(mlua::Value::Table(rows))
    }
}

// impl UserData for &Board {
//     fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
//         methods.add_method("get_piece", |lua, board, position| {
//             Ok(board.get_piece(position).unwrap().into_lua(lua))
//         });
//     }
// }
