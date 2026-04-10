use crate::common::rotation::Rotation;

/// Defines the set of possible actions that a player or an AI agent
/// can perform in the game environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Moves the current piece one column to the left.
    MoveLeft,
    /// Moves the current piece one column to the right.
    MoveRight,
    /// Moves the current piece one row down.
    MoveDown,
    /// Instantly drops the piece to the lowest possible position and locks it.
    HardDrop,
    /// Rotates the piece according to the specified rotation type.
    Rotate(Rotation),
    /// Swaps the current piece with the one in the "hold" slot.
    Hold,
}
