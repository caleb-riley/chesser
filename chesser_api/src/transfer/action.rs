use chesser_core::engine::{action::Action, color::PieceColor};
use serde::{Deserialize, Serialize};

use crate::transfer::PositionDto;

#[derive(Clone, Serialize, Deserialize)]
pub enum ActionDto {
    Relocate {
        origin: PositionDto,
        destination: PositionDto,
    },
    Spawn {
        position: PositionDto,
        id: String,
        color: String,
    },
    Deletion {
        position: PositionDto,
    },
}

impl From<&Action> for ActionDto {
    fn from(action: &Action) -> Self {
        match action {
            Action::Relocate {
                origin,
                destination,
            } => ActionDto::Relocate {
                origin: (*origin).into(),
                destination: (*destination).into(),
            },
            Action::Spawn {
                position,
                kind: id,
                color,
            } => ActionDto::Spawn {
                position: (*position).into(),
                id: id.into(),
                color: color.to_string(),
            },
            Action::Deletion { position } => ActionDto::Deletion {
                position: (*position).into(),
            },
        }
    }
}

impl From<ActionDto> for Action {
    fn from(action: ActionDto) -> Self {
        match action {
            ActionDto::Relocate {
                origin,
                destination,
            } => Self::Relocate {
                origin: origin.into(),
                destination: destination.into(),
            },
            ActionDto::Spawn {
                position,
                id,
                color,
            } => Self::Spawn {
                position: position.into(),
                kind: id,
                color: match color.as_str() {
                    "white" => PieceColor::White,
                    _ => PieceColor::Black,
                },
            },
            ActionDto::Deletion { position } => Self::Deletion {
                position: position.into(),
            },
        }
    }
}
