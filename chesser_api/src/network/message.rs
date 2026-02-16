use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

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

impl From<NetworkMessage> for Message {
    fn from(msg: NetworkMessage) -> Self {
        let text = serde_json::to_string(&msg).unwrap();
        Message::Text(text.into())
    }
}

impl TryFrom<Message> for NetworkMessage {
    type Error = serde_json::Error;

    fn try_from(msg: Message) -> Result<Self, Self::Error> {
        match msg {
            Message::Text(text) => serde_json::from_str(&text),
            _ => serde_json::from_str("null"),
        }
    }
}
