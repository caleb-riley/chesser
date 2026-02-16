use serde::{Deserialize, Serialize};

use crate::transfer::{MoveDto, PositionDto};

#[derive(Serialize, Deserialize)]
pub enum NetworkCommand {
    SendMove(MoveDto),
    RequestHints(PositionDto),
}
