use crate::{common::rotation::Rotation, engine::action::Action};

/// Represents a potential move evaluated by the AI agent.
///
/// A candidate consists of a target orientation and a horizontal position,
/// accompanied by a score representing the "fitness" of the resulting board state.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MoveCandidate {
    /// The target rotation state for the piece.
    pub rotation: Rotation,
    /// The horizontal displacement from the spawn position (negative for left, positive for right).
    pub column: i32,
    /// The heuristic score assigned to this move. Higher is better.
    pub score: f32,
}

impl MoveCandidate {
    /// Creates a new `MoveCandidate`.
    pub fn new(rotation: Rotation, column: i32, score: f32) -> Self {
        Self { rotation, column, score }
    }

    /// Converts the high-level move decision into a sequence of atomic game actions.
    ///
    /// The resulting vector contains the necessary rotation, followed by
    /// horizontal movements, and ends with a `HardDrop` to lock the piece.
    ///
    /// # Returns
    /// A `Vec<Action>` that, when executed by the engine, reaches the target state.
    pub fn to_sequence(&self) -> Vec<Action> {
        let count = self.column.abs() as usize;

        // Pre-allocating capacity for: 1 rotation + N movements + 1 hard drop.
        let mut sequence = Vec::with_capacity(2 + count);

        sequence.push(Action::Rotate(self.rotation));

        let movement = if self.column > 0 {
            Action::MoveRight
        } else {
            Action::MoveLeft
        };

        sequence.extend(std::iter::repeat(movement).take(count));
        sequence.push(Action::HardDrop);

        sequence
    }
}

impl Default for MoveCandidate {
    fn default() -> Self {
        Self { rotation: Rotation::None, column: 0, score: f32::NEG_INFINITY }
    }
}
