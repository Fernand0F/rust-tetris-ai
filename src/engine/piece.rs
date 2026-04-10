use crate::common::{vec2::Vec2, direction::Direction, rotation::Rotation};

/// Represents a Tetris piece consisting of several tiles and a pivot point.
///
/// The piece maintains both its current state and its original state to allow for
/// easy resetting when a new piece spawns.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Piece {
    /// The current position of the piece on the game board grid.
    pub pivot: Vec2,
    /// The positions of individual tiles relative to the pivot.
    pub tiles: Vec<Vec2>,
    /// The RGB color representation of the piece.
    pub color: (u8, u8, u8),

    /// The starting pivot position, used for resetting the piece.
    original_pivot: Vec2,
    /// The starting tile configuration, used for resetting the piece.
    original_tiles: Vec<Vec2>,
}

impl Piece {
    /// Creates a new `Piece` with a defined pivot, tile layout, and color.
    ///
    /// The initial state is stored internally as the "original" state for future resets.
    pub fn new(pivot: Vec2, tiles: &[Vec2], color: (u8, u8, u8)) -> Self {
        Self {
            pivot,
            tiles: tiles.to_vec(),
            color,
            original_pivot: pivot,
            original_tiles: tiles.to_vec(),
        }
    }

    /// Returns an iterator over the absolute positions of the tiles on the board.
    ///
    /// Each tile's local position is offset by the current `pivot` position.
    pub fn tiles(&self) -> impl Iterator<Item = Vec2> {
        self.tiles.iter().map(|&tile| tile + self.pivot)
    }

    /// Returns an iterator over the tiles' positions relative to the pivot.
    pub fn relative_tiles(&self) -> impl Iterator<Item = Vec2> {
        self.tiles.iter().copied()
    }

    /// Moves the piece one step in the specified direction.
    pub fn move_dir(&mut self, dir: Direction) {
        self.pivot += dir.to_vec2();
    }

    /// Rotates all tiles within the piece around the pivot point.
    pub fn rotate(&mut self, rot: Rotation) {
        self.tiles.iter_mut().for_each(|tile| tile.rotate(rot));
    }

    /// Resets the piece to its starting position and rotation.
    pub fn reset(&mut self) {
        self.pivot = self.original_pivot;
        self.tiles = self.original_tiles.clone();
    }
}
