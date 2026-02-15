use std::collections::HashMap;

use axum::{Json, Router, extract::State, response::IntoResponse, routing::get};
use chesser_api::{BoardDto, PieceConfigDto};

use crate::AppState;

async fn get_board(State(state): State<AppState>) -> impl IntoResponse {
    let game = state.game.lock().await;

    Json(BoardDto::from(&game.board))
}

async fn get_piece_configs(State(state): State<AppState>) -> impl IntoResponse {
    let game = state.game.lock().await;

    let mut configs = HashMap::new();

    for (id, config) in game.pieces.iter() {
        configs.insert(id.to_owned(), PieceConfigDto::from(config));
    }

    Json(configs)
}

pub fn game_router() -> Router<AppState> {
    Router::new()
        .route("/board", get(get_board))
        .route("/piece-configs", get(get_piece_configs))
}
