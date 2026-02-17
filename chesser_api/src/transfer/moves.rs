use chesser_core::engine::moves::Move;
use serde::{Deserialize, Serialize};

use crate::transfer::{ActionDto, PositionDto};

#[derive(Clone, Serialize, Deserialize)]
pub struct MoveDto {
    pub origin: PositionDto,
    pub destination: PositionDto,
    pub actions: Vec<ActionDto>,
    pub promotions: Vec<String>,
}

impl MoveDto {
    pub fn contains_deletion(&self) -> bool {
        self.actions
            .iter()
            .any(|a| matches!(a, ActionDto::Deletion { .. }))
    }
}

impl From<&Move> for MoveDto {
    fn from(the_move: &Move) -> Self {
        let mut actions = vec![];

        for action in &the_move.actions {
            actions.push((action).into());
        }

        Self {
            origin: (&the_move.origin).into(),
            destination: (&the_move.destination).into(),
            actions,
            promotions: the_move.promotions.clone(),
        }
    }
}

impl From<MoveDto> for Move {
    fn from(mv: MoveDto) -> Self {
        let mut actions = vec![];

        for action in mv.actions {
            actions.push(action.into());
        }

        Self::new(
            mv.origin.into(),
            mv.destination.into(),
            actions,
            mv.promotions,
        )
    }
}
