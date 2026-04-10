#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Rotation {
    /// No rotation applied.
    None,
    /// A 90-degree clockwise rotation.
    Clockwise,
    /// A 90-degree counter-clockwise rotation.
    CounterClockwise,
    /// A 180-degree rotation.
    Opposite,
}

impl Rotation {
    /// Returns an iterator over all possible rotation states.
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::None, Self::Clockwise, Self::CounterClockwise, Self::Opposite].into_iter()
    }
}
