use std::collections::HashMap;

use bevy::{
    asset::RenderAssetUsages,
    image::{CompressedImageFormats, ImageSampler, ImageType},
    prelude::*,
};
use bevy_egui::egui;

use chesser_api::{
    network::NetworkCommand,
    transfer::{ActionDto, BoardDto, MoveDto, PieceConfigDto, PieceDto, PositionDto},
};
use chesser_core::engine::color::PieceColor;

use crate::network::{NetworkClient, TokioRuntime, handle_network_messages, start_networking};

mod network;

pub fn play_move_sound(asset_server: &Res<AssetServer>, commands: &mut Commands, sound: &str) {
    let file_name = format!("sounds/{sound}.mp3");

    commands.spawn(AudioPlayer::new(asset_server.load(file_name)));
}

#[derive(Default, Resource)]
struct Interface {
    selection: Option<PositionDto>,
    board: Option<BoardDto>,
    pieces: HashMap<String, PieceConfigDto>,
    hints: HashMap<PositionDto, MoveDto>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_egui::EguiPlugin::default())
        .add_message::<UiEvent>()
        .add_systems(bevy_egui::EguiPrimaryContextPass, ui_system)
        .add_systems(Startup, (setup_camera_system, setup_textures_system))
        .add_systems(Startup, start_networking)
        .add_systems(Update, handle_ui_events)
        .add_systems(Update, handle_network_messages)
        .init_resource::<Interface>()
        .insert_resource(TokioRuntime::default())
        .insert_resource(PieceTextureIds {
            map: HashMap::new(),
        })
        .run();
}

fn send_move(the_move: MoveDto, net: &NetworkClient) {
    net.outgoing_tx
        .send(NetworkCommand::SendMove(the_move))
        .ok();
}

fn get_hints(position: PositionDto, net: &NetworkClient) {
    net.outgoing_tx
        .send(NetworkCommand::RequestHints(position))
        .ok();
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

const CELL_SIZE: f32 = 80.0;
const STROKE_WIDTH: f32 = 5.0;

#[derive(Resource, Default)]
struct PieceTextureIds {
    map: HashMap<String, egui::TextureId>,
}

fn setup_textures_system(
    mut textures: ResMut<PieceTextureIds>,
    mut contexts: bevy_egui::EguiContexts,
    mut images: ResMut<Assets<Image>>,
) {
    const PIECES_ROOT: &str = "./lua/pieces";

    for piece_dir in std::fs::read_dir(PIECES_ROOT).unwrap() {
        let piece_dir = piece_dir.unwrap();
        let piece_path = piece_dir.path();

        if !piece_path.is_dir() {
            continue;
        }

        let piece_name = piece_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap()
            .to_string();

        for color in ["white", "black"] {
            let file_path = piece_path.join(format!("assets/{color}_icon.png"));

            if !file_path.exists() {
                continue;
            }

            let bytes = std::fs::read(&file_path).expect("Failed to read image");

            let image = Image::from_buffer(
                &bytes,
                ImageType::Extension("png"),
                CompressedImageFormats::NONE,
                true,
                ImageSampler::default(),
                RenderAssetUsages::default(),
            )
            .expect("Failed to decode PNG");

            let handle = images.add(image);

            let texture_id = contexts.add_image(bevy_egui::EguiTextureHandle::Strong(handle));

            let piece_with_color = format!("{color}_{piece_name}");
            textures.map.insert(piece_with_color, texture_id);
        }
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
    position: PositionDto,
    interface: &mut Interface,
    asset_server: &Res<AssetServer>,
    net: &NetworkClient,
    commands: &mut Commands,
) {
    let Some(board) = &interface.board else {
        return;
    };

    if interface.selection.is_some() {
        interface.selection = None;

        if let Some(the_move) = interface.hints.get(&position) {
            let sound_file = if !the_move.contains_deletion() {
                match PieceColor::from_turn_count(board.turn_count) {
                    PieceColor::White => "move-self",
                    PieceColor::Black => "move-opponent",
                }
            } else {
                "capture"
            };

            send_move(the_move.clone(), net);

            play_move_sound(asset_server, commands, sound_file);
        }

        interface.hints.clear();
    } else if let Some(piece) = board.get_piece(position)
        && piece.color == PieceColor::from_turn_count(board.turn_count).to_string()
    {
        get_hints(position, net);

        interface.selection = Some(position);
    }
}

#[derive(Message, Clone)]
pub enum UiEvent {
    SquareClicked(PositionDto),
    SquareRightClicked(PositionDto),
}

fn _draw_player_info(ui: &mut egui::Ui, board: &BoardDto) {
    ui.label(
        egui::RichText::new(format!(
            "Current player: {}",
            PieceColor::from_turn_count(board.turn_count)
        ))
        .text_style(egui::TextStyle::Heading),
    );

    // ui.label(
    //     egui::RichText::new(format!(
    //         "White points: {}",
    //         game.board.captures[&PieceColor::White]
    //             .iter()
    //             .map(|k| game.pieces.get(k).unwrap().get_value())
    //             .sum::<i32>()
    //     ))
    //     .text_style(egui::TextStyle::Body),
    // );

    // ui.label(
    //     egui::RichText::new(format!(
    //         "Black points: {}",
    //         game.board.captures[&PieceColor::Black]
    //             .iter()
    //             .map(|k| game.pieces.get(k).unwrap().get_value())
    //             .sum::<i32>()
    //     ))
    //     .text_style(egui::TextStyle::Body),
    // );
}

fn allocate_board_cells(
    ui: &mut egui::Ui,
    board: &BoardDto,
) -> Vec<Vec<(egui::Rect, egui::Response)>> {
    let mut cells = vec![];

    for _ in 0..board.dimensions {
        let mut row_cells = vec![];

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
            for _ in 0..board.dimensions {
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
    position: PositionDto,
    board: &BoardDto,
    textures: &Res<PieceTextureIds>,
) {
    let base_color = if (position.row + position.column).is_multiple_of(2) {
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

    if let Some(piece) = board.get_piece(position) {
        draw_piece(ui, rect, piece, textures);
    }
}

fn paint_cell_strokes(
    ui: &egui::Ui,
    rect: egui::Rect,
    position: PositionDto,
    interface: &Interface,
    cells: &[Vec<(egui::Rect, egui::Response)>],
) {
    if interface.selection == Some(position) {
        ui.painter().rect_stroke(
            rect,
            CELL_SIZE / 2.0,
            egui::Stroke::new(STROKE_WIDTH, egui::Color32::GREEN),
            egui::StrokeKind::Inside,
        );
    }

    if let Some(the_move) = interface.hints.get(&position) {
        let stroke_color = match the_move.contains_deletion() {
            false => egui::Color32::YELLOW,
            true => egui::Color32::RED,
        };

        ui.painter().rect_stroke(
            rect,
            CELL_SIZE / 2.0,
            egui::Stroke::new(STROKE_WIDTH, stroke_color),
            egui::StrokeKind::Inside,
        );

        let target_cell = &cells[the_move.destination.row][the_move.destination.column];

        if the_move.contains_deletion() && target_cell.1.hovered() {
            for action in &the_move.actions {
                if let ActionDto::Relocate { destination, .. } = action {
                    let capture_cell = &cells[destination.row][destination.column];

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
}

fn draw_piece(ui: &egui::Ui, rect: egui::Rect, piece: &PieceDto, textures: &PieceTextureIds) {
    let icon_name = format!("{}_{}", piece.color, piece.kind);

    if let Some(texture_id) = textures.map.get(&icon_name) {
        draw_texture(ui, rect.center(), texture_id);
    }
}

fn ui_system(
    mut contexts: bevy_egui::EguiContexts,
    interface: ResMut<Interface>,
    textures: Res<PieceTextureIds>,
    mut ui_events: MessageWriter<UiEvent>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    if let Some(board_dto) = &interface.board {
        egui::Window::new("Chess").show(ctx, |ui| {
            // draw_player_info(ui, &game.inner);

            let original_spacing = ui.spacing_mut().item_spacing;
            ui.spacing_mut().item_spacing = egui::Vec2::ZERO;

            let cells = allocate_board_cells(ui, board_dto);

            for (row, cell_row) in cells.iter().enumerate() {
                for (column, (rect, response)) in cell_row.iter().enumerate() {
                    let position = PositionDto { row, column };

                    paint_cell_piece(ui, *rect, response, position, board_dto, &textures);
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
    }

    Ok(())
}

fn handle_ui_events(
    mut events: MessageReader<UiEvent>,
    mut interface: ResMut<Interface>,
    asset_server: Res<AssetServer>,
    net: Res<NetworkClient>,
    mut commands: Commands,
) {
    for event in events.read() {
        match event {
            UiEvent::SquareClicked(position) => {
                handle_click(
                    *position,
                    &mut interface,
                    &asset_server,
                    &net,
                    &mut commands,
                );
            }
            UiEvent::SquareRightClicked(_position) => {
                // if let Some(piece) = game.inner.board.get_piece(*position) {
                //     println!(
                //         "{} {{ color: {}, position: {} }}",
                //         piece.kind,
                //         piece.color,
                //         position.as_notation()
                //     );
                // }
            }
        }
    }
}
