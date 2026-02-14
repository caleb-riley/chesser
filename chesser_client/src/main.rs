use std::collections::HashMap;

use bevy::prelude::*;
use bevy_egui::egui;

use chesser_core::{
    game::Game,
    moves::{Move, MoveKind},
    piece::{Piece, PieceColor},
    position::Position,
};

pub fn play_move_sound(asset_server: &Res<AssetServer>, commands: &mut Commands, sound: &str) {
    let file_name = format!("sounds/{sound}.mp3");

    commands.spawn(AudioPlayer::new(asset_server.load(file_name)));
}

#[derive(Default, Resource)]
struct Interface {
    selection: Option<Position>,
    hints: HashMap<Position, Move>,
}

#[derive(Resource)]
struct GameResource {
    inner: Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_egui::EguiPlugin::default())
        .add_message::<UiEvent>()
        .add_systems(Startup, (setup_camera_system, setup_textures_system))
        .add_systems(bevy_egui::EguiPrimaryContextPass, ui_system)
        .add_systems(Update, handle_ui_events)
        .init_resource::<Interface>()
        .insert_resource(PieceTextureIds {
            map: HashMap::new(),
        })
        .insert_resource(GameResource {
            inner: {
                let mut game = Game::default();
                game.register_helpers().unwrap();
                game.load_piece_configs("./lua/pieces/");
                game
            },
        })
        .run();
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn darken(color: egui::Color32, factor: f32) -> egui::Color32 {
    let r = (color.r() as f32 * factor) as u8;
    let g = (color.g() as f32 * factor) as u8;
    let b = (color.b() as f32 * factor) as u8;

    egui::Color32::from_rgb(r, g, b)
}

const LIGHT_SQUARE: egui::Color32 = egui::Color32::from_rgb(235, 236, 208);
const DARK_SQUARE: egui::Color32 = egui::Color32::from_rgb(115, 149, 82);

const CELL_SIZE: f32 = 75.0;
const STROKE_WIDTH: f32 = 5.0;

#[derive(Resource, Default)]
struct PieceTextureIds {
    map: HashMap<String, egui::TextureId>,
}

fn setup_textures_system(
    mut textures: ResMut<PieceTextureIds>,
    mut contexts: bevy_egui::EguiContexts,
    asset_server: Res<AssetServer>,
) {
    register_piece_texture(&mut textures, &mut contexts, &asset_server, "pawn");
    register_piece_texture(&mut textures, &mut contexts, &asset_server, "knight");
    register_piece_texture(&mut textures, &mut contexts, &asset_server, "bishop");
    register_piece_texture(&mut textures, &mut contexts, &asset_server, "rook");
    register_piece_texture(&mut textures, &mut contexts, &asset_server, "king");
    register_piece_texture(&mut textures, &mut contexts, &asset_server, "queen");
}

fn register_piece_texture(
    textures: &mut ResMut<PieceTextureIds>,
    contexts: &mut bevy_egui::EguiContexts,
    asset_server: &Res<AssetServer>,
    kind: &str,
) {
    for color in ["white", "black"] {
        let name = format!("{}_{}", color, kind);
        let handle: Handle<Image> = asset_server.load(format!("pieces/{name}.png"));
        let id = contexts.add_image(bevy_egui::EguiTextureHandle::Strong(handle));

        textures.map.insert(name, id);
    }
}

fn draw_texture(ui: &egui::Ui, center: egui::Pos2, texture_id: &egui::TextureId) {
    let image_size = egui::Vec2::splat(CELL_SIZE * 0.8);
    let image_rect = egui::Rect::from_center_size(center, image_size);

    ui.painter().image(
        *texture_id,
        image_rect,
        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        egui::Color32::WHITE,
    );
}

fn handle_click(
    position: Position,
    interface: &mut ResMut<Interface>,
    game: &mut ResMut<GameResource>,
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
) {
    if let Some(old_selection) = interface.selection {
        interface.selection = None;

        if let Some(the_move) = interface.hints.get(&position) {
            let sound_file = match the_move.kind {
                MoveKind::Passive => {
                    match PieceColor::from_turn_count(game.inner.board.turn_count) {
                        PieceColor::White => "move-self",
                        PieceColor::Black => "move-opponent",
                    }
                }
                MoveKind::Capture(_) => "capture",
            };

            let the_move = the_move.clone();

            game.inner.board.perform_move(old_selection, &the_move);

            play_move_sound(asset_server, commands, sound_file);
        }

        interface.hints.clear();
    } else if let Some(piece) = game.inner.board.get_piece(position)
        && piece.color == PieceColor::from_turn_count(game.inner.board.turn_count)
    {
        let hints = game
            .inner
            .get_available_moves(&piece.kind, position)
            .into_iter()
            .map(|m| (m.destination, m))
            .collect();

        interface.selection = Some(position);
        interface.hints = hints;
    }
}

#[derive(Message, Debug, Clone)]
pub enum UiEvent {
    SquareClicked(Position),
    SquareRightClicked(Position),
}

fn draw_player_info(ui: &mut egui::Ui, game: &Game) {
    ui.label(
        egui::RichText::new(format!(
            "Current player: {}",
            PieceColor::from_turn_count(game.board.turn_count)
        ))
        .text_style(egui::TextStyle::Heading),
    );

    ui.label(
        egui::RichText::new(format!(
            "White points: {}",
            game.board.captures[&PieceColor::White]
                .iter()
                .map(|k| game.pieces.get(k).unwrap().get_value())
                .sum::<i32>()
        ))
        .text_style(egui::TextStyle::Body),
    );

    ui.label(
        egui::RichText::new(format!(
            "Black points: {}",
            game.board.captures[&PieceColor::Black]
                .iter()
                .map(|k| game.pieces.get(k).unwrap().get_value())
                .sum::<i32>()
        ))
        .text_style(egui::TextStyle::Body),
    );
}

fn allocate_board_cells(ui: &mut egui::Ui) -> Vec<Vec<(egui::Rect, egui::Response)>> {
    let mut cells = vec![];

    for _ in 0..8 {
        let mut row_cells = vec![];

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
            for _ in 0..8 {
                let (rect, response) =
                    ui.allocate_exact_size(egui::Vec2::splat(CELL_SIZE), egui::Sense::click());

                row_cells.push((rect, response));
            }
        });

        cells.push(row_cells);
    }

    cells
}

fn paint_cell_piece(
    ui: &egui::Ui,
    rect: egui::Rect,
    response: &egui::Response,
    position: Position,
    game: &Game,
    textures: &Res<PieceTextureIds>,
) {
    let base_color = if (position.row() + position.column()).is_multiple_of(2) {
        LIGHT_SQUARE
    } else {
        DARK_SQUARE
    };

    let color = if response.hovered() {
        darken(base_color, 0.8)
    } else {
        base_color
    };

    ui.painter().rect_filled(rect, 0.0, color);

    if let Some(piece) = game.board.get_piece(position) {
        draw_piece(ui, rect, piece, textures);
    }
}

fn paint_cell_strokes(
    ui: &egui::Ui,
    rect: egui::Rect,
    position: Position,
    game: &Interface,
    cells: &[Vec<(egui::Rect, egui::Response)>],
) {
    if game.selection == Some(position) {
        ui.painter().rect_stroke(
            rect,
            CELL_SIZE / 2.0,
            egui::Stroke::new(STROKE_WIDTH, egui::Color32::GREEN),
            egui::StrokeKind::Inside,
        );
    }

    if let Some(the_move) = game.hints.get(&position) {
        let stroke_color = match the_move.kind {
            MoveKind::Passive => egui::Color32::YELLOW,
            MoveKind::Capture(_) => egui::Color32::RED,
        };

        ui.painter().rect_stroke(
            rect,
            CELL_SIZE / 2.0,
            egui::Stroke::new(STROKE_WIDTH, stroke_color),
            egui::StrokeKind::Inside,
        );

        let target_cell = &cells[the_move.destination.row()][the_move.destination.column()];

        if let MoveKind::Capture(capture_positions) = &the_move.kind
            && target_cell.1.hovered()
        {
            for capture_pos in capture_positions {
                let capture_cell = &cells[capture_pos.row()][capture_pos.column()];

                ui.painter().rect_stroke(
                    capture_cell.0,
                    CELL_SIZE / 2.0,
                    egui::Stroke::new(STROKE_WIDTH, egui::Color32::BLACK),
                    egui::StrokeKind::Inside,
                );
            }
        }
    }
}

fn draw_piece(ui: &egui::Ui, rect: egui::Rect, piece: &Piece, textures: &PieceTextureIds) {
    let icon_name = format!("{}_{}", piece.color.text(), piece.kind);

    if let Some(texture_id) = textures.map.get(&icon_name) {
        draw_texture(ui, rect.center(), texture_id);
    }
}

fn ui_system(
    mut contexts: bevy_egui::EguiContexts,
    interface: ResMut<Interface>,
    game: ResMut<GameResource>,
    textures: Res<PieceTextureIds>,
    mut ui_events: MessageWriter<UiEvent>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("Chess").show(ctx, |ui| {
        draw_player_info(ui, &game.inner);

        let original_spacing = ui.spacing_mut().item_spacing;
        ui.spacing_mut().item_spacing = egui::Vec2::ZERO;

        let cells = allocate_board_cells(ui);

        for (row, cell_row) in cells.iter().enumerate() {
            for (column, (rect, response)) in cell_row.iter().enumerate() {
                let position = Position::new(row, column);

                paint_cell_piece(ui, *rect, response, position, &game.inner, &textures);
                paint_cell_strokes(ui, *rect, position, &interface, &cells);

                if response.clicked() {
                    ui_events.write(UiEvent::SquareClicked(position));
                }

                if response.secondary_clicked() {
                    ui_events.write(UiEvent::SquareRightClicked(position));
                }
            }
        }

        ui.spacing_mut().item_spacing = original_spacing;
    });

    Ok(())
}

fn handle_ui_events(
    mut events: MessageReader<UiEvent>,
    mut interface: ResMut<Interface>,
    mut game: ResMut<GameResource>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for event in events.read() {
        match event {
            UiEvent::SquareClicked(position) => {
                handle_click(
                    *position,
                    &mut interface,
                    &mut game,
                    &asset_server,
                    &mut commands,
                );
            }
            UiEvent::SquareRightClicked(position) => {
                if let Some(piece) = game.inner.board.get_piece(*position) {
                    println!(
                        "{} {{ color: {}, position: {} }}",
                        piece.kind,
                        piece.color,
                        position.as_notation()
                    );
                }
            }
        }
    }
}
