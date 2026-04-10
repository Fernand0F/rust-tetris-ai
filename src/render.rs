use macroquad::prelude::*;
use crate::engine::{core::TetrisEngine, piece::Piece};

/// Handles the visual representation of the Tetris game using the Macroquad engine.
///
/// It translates the logical coordinates of the `TetrisEngine` into pixel
/// coordinates on the screen, managing the board, UI boxes, and animations.
#[derive(Debug, Clone)]
pub struct TetrisRenderer<'a> {
    /// Horizontal screen offset for the board.
    pub offset_x: f32,
    /// Vertical screen offset for the board.
    pub offset_y: f32,
    /// Size of a single tile in pixels.
    pub tile_size: f32,
    /// Text styling for labels and score.
    text_params: TextParams<'a>,
}

impl<'a> TetrisRenderer<'a> {
    /// Asynchronously loads assets and initializes the renderer.
    pub async fn new(offset_x: f32, offset_y: f32, tile_size: f32) -> Self {
        let font = load_ttf_font("google-sans-code.ttf").await.unwrap();
        let text_params = TextParams {
            font: Some(Box::leak(Box::new(font))),
            font_size: 18,
            color: macroquad::color::WHITE,
            ..Default::default()
        };

        Self { offset_x, offset_y, tile_size, text_params }
    }

    /// Renders the complete game state, including the lattice, active piece,
    /// ghost piece, and UI elements.
    pub fn draw(&self, engine: &TetrisEngine) {
        let border_width = 2.0;
        let (w, h) = engine.dimensions;

        // Container
        self.draw_container(self.offset_x, self.offset_y, w as f32, h as f32, border_width);

        // Ghost piece
        let ghost_piece = engine.get_ghost_piece();
        self.draw_piece_in_game(&ghost_piece, 80);

        // Current piece
        self.draw_piece_in_game(&engine.current_piece, 255);

        // Lattice tiles
        for dx in 0..w {
            for dy in 0..h {
                if let Some(color) = engine.lattice.get_color((dx, dy)) {
                    let (r, g, b) = color;
                    let color = Color::from_rgba(r, g, b, 255);
                    self.draw_tile(dx as f32, dy as f32, color);
                }
            }
        }

        // Score
        draw_text_ex(
            &format!("Score: {}", engine.score),
            self.offset_x + (w as f32 + 1.0) * self.tile_size,
            self.offset_y + 1.0 * self.tile_size,
            self.text_params.clone(),
        );

        // Next
        self.draw_piece_box(&Some(engine.next_piece.clone()), "Next", w, 0.0, border_width);

        // Held
        self.draw_piece_box(&engine.held_piece, "Held", w, 7.0, border_width);
    }

    /// Draws a single tile at a grid position with a specific color.
    fn draw_tile(&self, x: f32, y: f32, color: Color) {
        draw_rectangle(
            self.offset_x + x * self.tile_size,
            self.offset_y + y * self.tile_size,
            self.tile_size,
            self.tile_size,
            color,
        );
    }

    /// Renders a piece directly on the game board.
    ///
    /// Uses an `alpha` value to handle transparency for the ghost piece.
    fn draw_piece_in_game(&self, piece: &Piece, alpha: u8) {
        for tile in piece.tiles() {
            let (r, g, b) = piece.color;
            let color = Color::from_rgba(r, g, b, alpha);
            self.draw_tile(tile.x, tile.y, color);
        }
    }

    /// Draws a specialized UI container for previewing pieces (Next and Held).
    fn draw_container(&self, x: f32, y: f32, w: f32, h: f32, border_width: f32) {
        draw_rectangle(
            x - border_width, y - border_width,
            w * self.tile_size + border_width * 2.0,
            h * self.tile_size + border_width * 2.0,
            WHITE,
        );
        draw_rectangle(x, y, w as f32 * self.tile_size, h as f32 * self.tile_size, BLACK);
    }

    /// Draws a specialized UI box to display pieces outside the main grid.
    ///
    /// This is used for the "Next" and "Held" piece previews. It renders a label,
    /// a decorative container, and the piece itself if it exists.
    fn draw_piece_box(
        &self,
        piece: &Option<Piece>,
        label: &str,
        w: usize,
        offset_y: f32,
        border_width: f32,
    ) {
        draw_text_ex(
            label,
            self.offset_x + (w as f32 + 2.1) * self.tile_size,
            self.offset_y + (2.6 + offset_y) * self.tile_size,
            self.text_params.clone(),
        );
        self.draw_container(
            self.offset_x + (w as f32 + 0.8) * self.tile_size,
            self.offset_y + (3.3 + offset_y) * self.tile_size,
            4.4, 4.4,
            border_width,
        );
        if let Some(piece) = piece {
            self.draw_piece(piece, w as f32 + 2.5, 5.0 + offset_y);
        }
    }

    /// Renders a piece at a specific screen offset using its relative tile coordinates.
    ///
    /// Unlike drawing during gameplay, this method ignores the piece's `pivot`
    /// and uses the provided `offset_x` and `offset_y` as the new origin.
    fn draw_piece(&self, piece: &Piece, offset_x: f32, offset_y: f32) {
        for tile in piece.relative_tiles() {
            let (r, g, b) = piece.color;
            let color = Color::from_rgba(r, g, b, 255);
            self.draw_tile(tile.x + offset_x, tile.y + offset_y, color);
        }
    }
}
