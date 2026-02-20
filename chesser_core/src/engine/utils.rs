use std::collections::HashSet;

use crate::engine::{
    action::Action,
    board::Board,
    moves::Move,
    piece::Piece,
    position::{Offset, Position},
};

pub const PERPENDICULAR: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
pub const DIAGONAL: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];
pub const STAGGERED: [(isize, isize); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];

pub fn available_positions_in_directions(
    position: Position,
    piece: &Piece,
    board: &Board,
    directions: &[(isize, isize)],
) -> HashSet<Position> {
    let mut positions = HashSet::new();

    for (delta_row, delta_column) in directions {
        let mut scale = 1;

        while let Some(target_position) = position.offset_by_checked(
            Offset::new(*delta_row, *delta_column).scale_by(scale),
            board.dimensions(),
        ) {
            if let Some(target) = board.get_piece_at_position(target_position) {
                if target.color != piece.color {
                    positions.insert(target_position);
                }

                break;
            } else {
                positions.insert(target_position);
            }

            scale += 1;
        }
    }

    positions
}

pub fn generate_moves(
    positions: HashSet<Position>,
    origin: Position,
    piece: &Piece,
    board: &Board,
) -> Vec<Move> {
    let mut moves = vec![];

    for position in positions {
        if let Some(target) = board.get_piece_at_position(position) {
            if target.color != piece.color {
                moves.push(Move::new(
                    origin,
                    position,
                    vec![
                        Action::Deletion { position },
                        Action::Relocate {
                            origin,
                            destination: position,
                        },
                    ],
                    vec![],
                ));
            }
        } else {
            moves.push(Move::new(
                origin,
                position,
                vec![Action::Relocate {
                    origin,
                    destination: position,
                }],
                vec![],
            ));
        }
    }

    moves
}
