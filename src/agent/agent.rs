use crate::{agent::{eval::{compute_bumpiness, compute_aggregate_height, count_holes, get_column_heights}, move_candidate::MoveCandidate}, common::rotation::Rotation, engine::{action::Action, core::TetrisEngine}};

/// An automated agent that uses heuristic evaluation and lookahead search
/// to play Tetris.
///
/// The agent evaluates board states using a linear combination of features
/// (holes, height, bumpiness) weighted by the provided parameters.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TetrisAgent {
    /// Penalty weight for holes in the lattice. Usually negative.
    pub holes_w: f32,
    /// Penalty weight for the total aggregate height. Usually negative.
    pub height_w: f32,
    /// Penalty weight for surface unevenness. Usually negative.
    pub bumpiness_w: f32,
    /// Number of future pieces to simulate. Increasing this exponentially
    /// improves play quality but increases computation time.
    pub lookahead_depth: u8,
}

impl TetrisAgent {
    /// Determines the best sequence of actions to take for the current board state.
    pub fn get_action(&self, engine: &TetrisEngine) -> Vec<Action> {
        let (candidate, used_hold) = self.lookahead(engine, self.lookahead_depth);

        if used_hold {
            [vec![Action::Hold], candidate.to_sequence()].concat()
        } else {
            candidate.to_sequence()
        }
    }

    /// Heuristic function that calculates the "value" of a board state.
    fn evaluate_game_state(&self, engine: &TetrisEngine) -> f32 {
        let holes = count_holes(engine) as f32;
        let column_heights = get_column_heights(engine);
        let height = compute_aggregate_height(&column_heights) as f32;
        let bumpiness = compute_bumpiness(&column_heights) as f32;

        holes * self.holes_w + height * self.height_w + bumpiness * self.bumpiness_w
    }

    fn lookahead(&self, engine: &TetrisEngine, depth: u8) -> (MoveCandidate, bool) {
        let mut engine = engine.clone();

        // Current piece
        let current_piece_candidate = self.raw_lookahead(&engine, depth);

        // Held piece
        let _ = engine.step(Action::Hold);
        let held_piece_candidate = self.raw_lookahead(&engine, depth);

        if current_piece_candidate.score > held_piece_candidate.score {
            (current_piece_candidate, false)
        } else {
            (held_piece_candidate, true)
        }
    }

    /// Explores the state space of all possible rotations and columns.
    ///
    /// This method simulates dropping the piece in every valid position
    /// and uses recursion to evaluate subsequent pieces up to `depth`.
    fn raw_lookahead(&self, engine: &TetrisEngine, depth: u8) -> MoveCandidate {
        let mut best_candidate = MoveCandidate::default();

        for rot in Rotation::iter() {
            let mut rotation_state = engine.clone(); /* Clone engine state for backtracking */
            let _ = rotation_state.step(Action::Rotate(rot)); /* Apply a posible rotation */

            // Position piece in the extreme left
            let mut column = 0;
            while rotation_state.step(Action::MoveLeft).is_ok() {
                column -= 1;
            }

            loop {
                let mut current = rotation_state.clone(); /* Clone engine state for backtracking */
                let _ = current.step(Action::HardDrop); /* Hard drop piece  */

                let score = if depth > 0 {
                    let (lookahead_candidate, _) = self.lookahead(&current, depth - 1);
                    lookahead_candidate.score
                } else {
                    self.evaluate_game_state(&current)
                };

                if score > best_candidate.score {
                    best_candidate = MoveCandidate::new(rot, column, score);
                }

                // Stop if the piece reach the right wall.
                if rotation_state.step(Action::MoveRight).is_err() {
                    break;
                }

                column += 1;
            }
        }

        best_candidate
    }
}
