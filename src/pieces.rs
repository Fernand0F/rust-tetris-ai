use crate::{common::vec2::vec2, engine::piece::Piece};

/// Generates the standard set of seven Tetris pieces.
pub fn get_pieces(width: usize) -> Vec<Piece> {
    let base_x = width as f32 / 2.0;

    vec![
        // I-PIECE
        Piece::new(
            vec2(base_x - 0.5, 0.5),
            &[
                vec2(-1.5, 0.5),
                vec2(-0.5, 0.5),
                vec2(0.5, 0.5),
                vec2(1.5, 0.5),
            ],
            (111, 231, 255) // #6FE7FF
        ),

        // T-PIECE
        Piece::new(
            vec2(base_x, 1.0),
            &[
                vec2(0.0, 0.0),
                vec2(1.0, 0.0),
                vec2(-1.0, 0.0),
                vec2(0.0, -1.0),
            ],
            (210, 150, 255) // #D296FF
        ),

        // O-PIECE
        Piece::new(
            vec2(base_x - 0.5, 0.5),
            &[
                vec2(0.5, -0.5),
                vec2(0.5, 0.5),
                vec2(-0.5, -0.5),
                vec2(-0.5, 0.5),
            ],
            (255, 239, 148) // #FFEF94
        ),

        // L-PIECE
        Piece::new(
            vec2(base_x - 1.0, 1.0),
            &[
                vec2(0.0, 0.0),
                vec2(-1.0, 0.0),
                vec2(1.0, 0.0),
                vec2(1.0, -1.0),
            ],
            (255, 182, 115) // #FFB673
        ),

        // J-PIECE
        Piece::new(
            vec2(base_x - 1.0, 1.0),
            &[
                vec2(0.0, 0.0),
                vec2(-1.0, 0.0),
                vec2(1.0, 0.0),
                vec2(-1.0, -1.0),
            ],
            (115, 175, 255) // #73AFFF
        ),

        // S-PIECE
        Piece::new(
            vec2(base_x, 1.0),
            &[
                vec2(0.0, 0.0),
                vec2(0.0, -1.0),
                vec2(1.0, -1.0),
                vec2(-1.0, 0.0),
            ],
            (138, 255, 170) // #8AFFAA
        ),

        // Z-PIECE
        Piece::new(
            vec2(base_x, 1.0),
            &[
                vec2(0.0, 0.0),
                vec2(0.0, 1.0),
                vec2(-1.0, 1.0),
                vec2(1.0, 0.0),
            ],
            (255, 111, 111) // #FF6F6F
        ),
    ]
}
