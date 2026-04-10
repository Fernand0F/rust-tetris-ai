use crate::engine::piece::Piece;

/// Represents the fixed grid (board) of the Tetris game.
///
/// The lattice tracks which cells are occupied by fallen pieces and their colors.
/// It uses a coordinate system where (0,0) is the top-left corner.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Lattice {
    width: usize,
    height: usize,
    /// A 2D grid of tiles. `None` represents an empty cell,
    /// while `Some(color)` represents an occupied cell with its RGB color.
    tiles: Vec<Vec<Option<(u8, u8, u8)>>>,
}

impl Lattice {
    /// Creates a new empty `Lattice` with the specified dimensions.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width, height,
            tiles: vec![vec![None; width]; height],
        }
    }

    /// Checks if a piece is colliding with any existing tiles or board boundaries.
    ///
    /// Returns `true` if any part of the piece is out of bounds or overlaps
    /// with an occupied cell.
    pub fn check_collision(&self, piece: &Piece) -> bool {
        piece.tiles()
            .any(|tile| {
                if let Some((x, y)) = tile.to_grid_pos() {
                    x >= self.width || y >= self.height || self.is_occupied(x, y)
                } else {
                    // Treat invalid grid positions (negative coordinates) as collisions
                    true
                }
            })
    }

    /// Validates if the piece has crossed the left wall boundary.
    pub fn check_left_wall_collision(&self, piece: &Piece) -> bool {
        piece.tiles().any(|tile| tile.x < 0.0)
    }

    /// Validates if the piece has crossed the right wall boundary.
    pub fn check_right_wall_collision(&self, piece: &Piece) -> bool {
        piece.tiles().any(|tile| tile.x as usize >= self.width)
    }

    /// Validates if the piece has crossed the ceiling (top) boundary.
    pub fn check_ceil_collision(&self, piece: &Piece) -> bool {
        piece.tiles().any(|tile| tile.y < 0.0)
    }

    /// Checks specifically for collisions with tiles already placed on the lattice.
    pub fn check_tile_collision(&self, piece: &Piece) -> bool {
        piece.tiles()
            .any(|tile| {
                tile.to_grid_pos()
                    .map(|(x, y)| self.is_occupied(x, y))
                    .unwrap_or(false)
            })
    }

    /// Places a colored tile at the specified coordinates.
    ///
    /// # Errors
    /// Returns an `Err` if the coordinates are outside the lattice boundaries.
    pub fn put(&mut self, (x, y): (usize, usize), color: (u8, u8, u8)) -> Result<(), String> {
        if x >= self.width || y >= self.height {
            Err(format!("Index out of bounds: ({x},{y})"))
        } else {
            self.tiles[y][x] = Some(color);
            Ok(())
        }
    }

    /// Checks if a specific cell is occupied.
    ///
    /// Cells outside the lattice are not occupied (`false`) by default.
    pub fn is_occupied(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height && self.tiles[y][x].is_some()
    }

    /// Retrieves the color of a tile at the given position, if it exists.
    pub fn get_color(&self, (x, y): (usize, usize)) -> Option<(u8, u8, u8)> {
        if x < self.width && y < self.height {
            self.tiles[y][x]
        } else {
            None
        }
    }

    /// Identifies and removes all fully occupied rows from the lattice.
    ///
    /// After removal, all rows above the cleared ones are shifted down.
    /// Returns the total number of rows cleared.
    pub fn clear_complete_rows(&mut self) -> usize {
        // Keep only rows that have at least one empty (None) cell
        self.tiles.retain(|row| row.iter().any(Option::is_none));

        let rows_cleared = self.height - self.tiles.len();

        // Re-fill the top with empty rows to maintain lattice height
        while self.tiles.len() < self.height {
            self.tiles.insert(0, vec![None; self.width]);
        }

        rows_cleared
    }
}
