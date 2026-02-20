use std::collections::HashMap;

use axum::{Json, Router, extract::State, response::IntoResponse, routing::get};
use chesser_api::transfer::{BoardDto, PieceConfigDto};

use crate::AppState;

async fn get_board(State(state): State<AppState>) -> impl IntoResponse {
    let game = state.game.lock().await;

    Json(BoardDto::from(&game.board))
}

async fn get_piece_configs(State(state): State<AppState>) -> impl IntoResponse {
    let game = state.game.lock().await;

    let mut piece_configs = HashMap::new();

    for (piece_id, piece_config) in game.piece_configs() {
        piece_configs.insert(piece_id.to_owned(), PieceConfigDto::from(piece_config));
    }

    Json(piece_configs)
}

pub fn game_router() -> Router<AppState> {
    Router::new()
        .route("/board", get(get_board))
        .route("/piece-configs", get(get_piece_configs))
}
