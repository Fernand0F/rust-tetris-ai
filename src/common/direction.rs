use crate::common::vec2::{Vec2, vec2};

/// Represents the four cardinal directions.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction { Up, Down, Left, Right }

impl Direction {
    /// Converts the direction into a normalized `Vec2` unit vector.
    ///
    /// The Y-axis increases downwards (e.g., `Down` is `(0, 1)`).
    pub fn to_vec2(self) -> Vec2 {
        match self {
            Self::Up    => vec2(0.0, -1.0),
            Self::Down  => vec2(0.0, 1.0),
            Self::Left  => vec2(-1.0, 0.0),
            Self::Right => vec2(1.0, 0.0)
        }
    }
}
