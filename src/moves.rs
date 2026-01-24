use crate::position::Position;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum MoveKind {
    Passive,
    Capture(Vec<Position>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Move {
    pub destination: Position,
    pub kind: MoveKind,
}

impl Move {
    pub fn new(destination: Position, kind: MoveKind) -> Self {
        Self { destination, kind }
    }
}
