use std::collections::HashMap;

use bevy::prelude::*;
use bevy_egui::{
    EguiContexts, EguiPlugin, EguiPrimaryContextPass,
    egui::{Align2, Color32, RichText, Sense, Stroke, StrokeKind, TextStyle, Vec2, Window},
};

use crate::{
    board::Board,
    moves::{Move, MoveKind},
    piece::{PieceColor, PieceKind},
    position::Position,
};

mod board;
mod moves;
mod piece;
mod position;

#[derive(Resource)]
struct GameInfo {
    board: Board,
    selection: Option<Position>,
    hints: HashMap<Position, Move>,
}

impl Default for GameInfo {
    fn default() -> Self {
        Self {
            board: Board::standard(),
            selection: None,
            hints: HashMap::new(),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, setup_camera_system)
        .add_systems(EguiPrimaryContextPass, display_board)
        .init_resource::<GameInfo>()
        .run();
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn darken(color: Color32, factor: f32) -> Color32 {
    let r = (color.r() as f32 * factor) as u8;
    let g = (color.g() as f32 * factor) as u8;
    let b = (color.b() as f32 * factor) as u8;

    Color32::from_rgb(r, g, b)
}

const LIGHT_SQUARE: Color32 = Color32::from_rgb(235, 236, 208);
const DARK_SQUARE: Color32 = Color32::from_rgb(115, 149, 82);

const WHITE_PIECE: Color32 = Color32::from_rgb(249, 249, 249);
const BLACK_PIECE: Color32 = Color32::from_rgb(92, 89, 87);

const CELL_SIZE: f32 = 60.0;

fn display_board(mut contexts: EguiContexts, mut game: ResMut<GameInfo>) -> Result {
    Window::new("Chess").show(contexts.ctx_mut()?, |ui| {
        ui.label(
            RichText::new(format!(
                "Current player: {}",
                PieceColor::from_turn_count(game.board.turn_count)
            ))
            .text_style(TextStyle::Heading),
        );

        ui.label(
            RichText::new(format!(
                "White points: {}",
                game.board.captures[&PieceColor::White]
                    .iter()
                    .map(PieceKind::value)
                    .sum::<usize>()
            ))
            .text_style(TextStyle::Body),
        );

        ui.label(
            RichText::new(format!(
                "Black points: {}",
                game.board.captures[&PieceColor::Black]
                    .iter()
                    .map(PieceKind::value)
                    .sum::<usize>()
            ))
            .text_style(TextStyle::Body),
        );

        let original_spacing = ui.spacing_mut().item_spacing;
        ui.spacing_mut().item_spacing = Vec2::ZERO;

        for row in 0..8 {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::ZERO;

                for col in 0..8 {
                    let position = Position::new(row, col);

                    let base_color = if (row + col) % 2 == 0 {
                        LIGHT_SQUARE
                    } else {
                        DARK_SQUARE
                    };

                    let (rect, response) =
                        ui.allocate_exact_size(Vec2::splat(CELL_SIZE), Sense::click());

                    let color = if response.hovered() {
                        darken(base_color, 0.8)
                    } else {
                        base_color
                    };

                    ui.painter().rect_filled(rect, 0.0, color);

                    if game.selection == Some(position) {
                        ui.painter().rect_stroke(
                            rect,
                            0.0,
                            Stroke::new(4.0, Color32::GREEN),
                            StrokeKind::Inside,
                        );
                    }

                    if let Some(the_move) = game.hints.get(&position) {
                        let stroke_color = if the_move.kind() == MoveKind::Passive {
                            Color32::YELLOW
                        } else {
                            Color32::RED
                        };

                        ui.painter().rect_stroke(
                            rect,
                            0.0,
                            Stroke::new(4.0, stroke_color),
                            StrokeKind::Inside,
                        );
                    }

                    if let Some(piece) = game.board.get_piece(position) {
                        let center = rect.center();
                        let radius = CELL_SIZE * 0.25;

                        let (circle_color, text_color) = if piece.color == PieceColor::White {
                            (WHITE_PIECE, BLACK_PIECE)
                        } else {
                            (BLACK_PIECE, WHITE_PIECE)
                        };

                        ui.painter().circle_filled(center, radius, circle_color);

                        ui.painter().circle_stroke(
                            center,
                            radius,
                            Stroke::new(1.0, Color32::BLACK),
                        );

                        let text = piece.kind.label();

                        ui.painter().text(
                            center,
                            Align2::CENTER_CENTER,
                            text,
                            TextStyle::Body.resolve(ui.style()),
                            text_color,
                        );
                    }

                    if response.clicked() {
                        if let Some(old_selection) = game.selection {
                            game.selection = None;

                            if let Some(&the_move) = game.hints.get(&position) {
                                game.board.perform_move(old_selection, the_move);
                            }

                            game.hints.clear();
                        } else if let Some(piece) = game.board.get_piece(position).cloned()
                            && piece.color == PieceColor::from_turn_count(game.board.turn_count)
                        {
                            game.selection = Some(position);

                            game.hints.clear();
                            game.board.available_moves(&piece).iter().for_each(|m| {
                                game.hints.insert(m.destination(), *m);
                            });
                        }
                    }
                }
            });
        }

        ui.spacing_mut().item_spacing = original_spacing;
    });

    Ok(())
}
