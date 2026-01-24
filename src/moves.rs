use crate::position::Position;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MoveKind {
    Passive,
    Capture(Position),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Move {
    destination: Position,
    kind: MoveKind,
}

impl Move {
    pub fn new(destination: Position, kind: MoveKind) -> Self {
        Self { destination, kind }
    }

    pub fn destination(&self) -> Position {
        self.destination
    }

    pub fn kind(&self) -> MoveKind {
        self.kind
    }
}
