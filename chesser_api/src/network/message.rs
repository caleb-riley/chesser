use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::transfer::{BoardDto, MoveDto, PieceConfigDto};

#[derive(Serialize, Deserialize)]
pub enum NetworkMessage {
    BoardLoaded(BoardDto),
    PieceConfigsLoaded(HashMap<String, PieceConfigDto>),
    SocketMessage(String),
    HintsReceived(Vec<MoveDto>),
    MovePerformed,
    ExperiencedError(String),
}
