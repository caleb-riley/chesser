// use std::{collections::HashSet, sync::Arc};

// use crate::{
//     board::Board,
//     moves::{Move, MoveKind},
//     piece::{Piece, PieceColor},
//     position::Offset,
//     utils,
// };

// pub trait PieceKind: Send + Sync + 'static {
//     fn name(&self) -> &str;

//     fn value(&self) -> i32;

//     fn asset(&self) -> &str;

//     fn available_moves(&self, piece: &Piece, board: &Board) -> HashSet<Move>;

//     // fn post_move_hook(&self, _piece: &Piece, _board: &Board) {}
// }

// pub type PieceKindRef = Arc<dyn PieceKind>;

// #[derive(Debug)]
// pub struct Pawn;

// impl PieceKind for Pawn {
//     fn name(&self) -> &str {
//         "Pawn"
//     }

//     fn value(&self) -> i32 {
//         1
//     }

//     fn asset(&self) -> &str {
//         "pawn"
//     }

//     fn available_moves(&self, piece: &Piece, board: &Board) -> HashSet<Move> {
//         let mut moves = HashSet::new();

//         let delta_row = (piece.color == PieceColor::Black) as isize * 2 - 1;

//         if let Some(forward_one) = piece
//             .position
//             .offset_by(Offset::new(delta_row, 0))
//             .restrict_to(board.dimensions)
//             && board.get_piece(forward_one).is_none()
//         {
//             moves.insert(Move::new(forward_one, MoveKind::Passive));
//         }

//         if piece.history.is_empty()
//             && let Some(forward_two) = piece
//                 .position
//                 .offset_by(Offset::new(delta_row, 0).scale_by(2))
//                 .restrict_to(board.dimensions)
//             && board.get_piece(forward_two).is_none()
//         {
//             moves.insert(Move::new(forward_two, MoveKind::Passive));
//         }

//         for delta_column in [-1isize, 1isize] {
//             if let Some(diagonal_position) = piece
//                 .position
//                 .offset_by(Offset::new(delta_row, delta_column))
//                 .restrict_to(board.dimensions)
//             {
//                 if let Some(target) = board.get_piece(diagonal_position)
//                     && target.color != piece.color
//                 {
//                     moves.insert(Move::new(
//                         diagonal_position,
//                         MoveKind::Capture(vec![diagonal_position]),
//                     ));
//                 }

//                 let lateral_position = piece.position.offset_by(Offset::new(0, delta_column));

//                 if let Some(target) = board.get_piece(lateral_position)
//                     && target.color != piece.color
//                     && let Some(previous) = target.history.iter().next_back()
//                     && previous.offset_to(target.position).taxicab_magnitude() == 2
//                     && target.last_moved == Some(board.turn_count)
//                 {
//                     moves.insert(Move::new(
//                         diagonal_position,
//                         MoveKind::Capture(vec![lateral_position]),
//                     ));
//                 }
//             }
//         }

//         moves
//     }
// }

// #[derive(Debug)]
// pub struct Knight;

// impl PieceKind for Knight {
//     fn name(&self) -> &str {
//         "Knight"
//     }

//     fn value(&self) -> i32 {
//         3
//     }

//     fn asset(&self) -> &str {
//         "knight"
//     }

//     fn available_moves(&self, piece: &Piece, board: &Board) -> HashSet<Move> {
//         let mut moves = HashSet::new();

//         for (delta_row, delta_column) in utils::STAGGERED {
//             let Some(target_position) = piece
//                 .position
//                 .offset_by(Offset::new(delta_row, delta_column))
//                 .restrict_to(board.dimensions)
//             else {
//                 continue;
//             };

//             if let Some(target) = board.get_piece(target_position) {
//                 if target.color != piece.color {
//                     moves.insert(Move::new(
//                         target_position,
//                         MoveKind::Capture(vec![target_position]),
//                     ));
//                 }
//             } else {
//                 moves.insert(Move::new(target_position, MoveKind::Passive));
//             }
//         }

//         moves
//     }
// }

// #[derive(Debug)]
// pub struct Bishop;

// impl PieceKind for Bishop {
//     fn name(&self) -> &str {
//         "Bishop"
//     }

//     fn value(&self) -> i32 {
//         3
//     }

//     fn asset(&self) -> &str {
//         "bishop"
//     }

//     fn available_moves(&self, piece: &Piece, board: &Board) -> HashSet<Move> {
//         let positions = utils::available_positions_in_directions(piece, board, &utils::DIAGONAL);

//         utils::generate_moves(positions, piece, board)
//     }
// }

// #[derive(Debug)]
// pub struct Rook;

// impl PieceKind for Rook {
//     fn name(&self) -> &str {
//         "Rook"
//     }

//     fn value(&self) -> i32 {
//         5
//     }

//     fn asset(&self) -> &str {
//         "rook"
//     }

//     fn available_moves(&self, piece: &Piece, board: &Board) -> HashSet<Move> {
//         let positions =
//             utils::available_positions_in_directions(piece, board, &utils::PERPENDICULAR);

//         utils::generate_moves(positions, piece, board)
//     }
// }

// #[derive(Debug)]
// pub struct King;

// impl PieceKind for King {
//     fn name(&self) -> &str {
//         "King"
//     }

//     fn value(&self) -> i32 {
//         0
//     }

//     fn asset(&self) -> &str {
//         "king"
//     }

//     fn available_moves(&self, piece: &Piece, board: &Board) -> HashSet<Move> {
//         let mut moves = HashSet::new();

//         let neighbor_offsets = utils::PERPENDICULAR
//             .into_iter()
//             .chain(utils::DIAGONAL)
//             .collect::<Vec<_>>();

//         for (delta_row, delta_column) in neighbor_offsets {
//             let Some(target_position) = piece
//                 .position
//                 .offset_by(Offset::new(delta_row, delta_column))
//                 .restrict_to(board.dimensions)
//             else {
//                 continue;
//             };

//             if let Some(target) = board.get_piece(target_position) {
//                 if target.color != piece.color {
//                     moves.insert(Move::new(
//                         target_position,
//                         MoveKind::Capture(vec![target_position]),
//                     ));
//                 }
//             } else {
//                 moves.insert(Move::new(target_position, MoveKind::Passive));
//             }
//         }

//         moves
//     }
// }

// #[derive(Debug)]
// pub struct Queen;

// impl PieceKind for Queen {
//     fn name(&self) -> &str {
//         "Queen"
//     }

//     fn value(&self) -> i32 {
//         9
//     }

//     fn asset(&self) -> &str {
//         "queen"
//     }

//     fn available_moves(&self, piece: &Piece, board: &Board) -> HashSet<Move> {
//         let perp_positions =
//             utils::available_positions_in_directions(piece, board, &utils::PERPENDICULAR);
//         let diag_positions =
//             utils::available_positions_in_directions(piece, board, &utils::DIAGONAL);

//         let positions = perp_positions.into_iter().chain(diag_positions).collect();

//         utils::generate_moves(positions, piece, board)
//     }
// }
