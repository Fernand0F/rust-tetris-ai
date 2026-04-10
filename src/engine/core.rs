use rand::{rngs::ThreadRng, seq::IndexedRandom};
use crate::{common::{direction::Direction, rotation::Rotation}, engine::{action::Action, lattice::Lattice, piece::Piece}};

/// The core engine of the Tetris game, managing state, physics, and rules.
#[derive(Debug, Clone)]
pub struct TetrisEngine {
    /// The dimensions (width, height) of the playing field.
    pub dimensions: (usize, usize),
    /// The grid containing all settled tiles.
    pub lattice: Lattice,

    /// The pool of available piece templates.
    piece_pool: Vec<Piece>,
    /// The piece currently being controlled by the player/agent.
    pub current_piece: Piece,
    /// The piece that will spawn after the current one is placed.
    pub next_piece: Piece,
    /// An optional piece stored in the hold slot.
    pub held_piece: Option<Piece>,

    /// The current score, increased by clearing rows.
    pub score: usize,
    /// Flag indicating if the game has reached a terminal state.
    pub game_over: bool,

    /// Random number generator for spawning pieces.
    rng: ThreadRng,
}

impl TetrisEngine {
    /// Initializes a new game engine with specific dimensions and piece types.
    pub fn new((width, height): (usize, usize), piece_pool: &[Piece], mut rng: ThreadRng) -> Self {
        let current_piece = piece_pool.choose(&mut rng).cloned().expect("Empty piece pool");
        let next_piece = piece_pool.choose(&mut rng).cloned().expect("Empty piece pool");

        Self {
            dimensions: (width, height),
            lattice: Lattice::new(width, height),
            piece_pool: piece_pool.to_vec(),
            current_piece,
            next_piece,
            held_piece: None,
            score: 0,
            game_over: false,
            rng,
        }
    }

    /// Progresses the game state by one discrete action.
    ///
    /// Returns `Ok(())` if the action was processed, or `Err(())` if the
    /// game is already over or the movement is invalid.
    pub fn step(&mut self, action: Action) -> Result<(), ()>{
        if self.game_over {
            return Err(());
        }

        match action {
            Action::MoveLeft => self.try_move(Direction::Left)?,
            Action::MoveRight => self.try_move(Direction::Right)?,
            Action::MoveDown => self.try_move_down(),
            Action::HardDrop => self.hard_drop(),
            Action::Rotate(rot) => self.rotate_piece(rot),
            Action::Hold => self.hold(),
        }

        Ok(())
    }

    /// Calculates the "Ghost Piece", representing where the current piece
    /// would land if dropped immediately.
    pub fn get_ghost_piece(&self) -> Piece {
        let mut ghost = self.current_piece.clone();
        loop {
            ghost.move_dir(Direction::Down);
            if self.lattice.check_collision(&ghost) {
                ghost.move_dir(Direction::Up);
                return ghost;
            }
        }
    }

    /// Attempts to move the current piece in a given direction.
    ///
    /// If a collision is detected, the movement is rolled back and returns `Err(())`.
    fn try_move(&mut self, dir: Direction) -> Result<(), ()> {
        self.current_piece.move_dir(dir);

        if self.lattice.check_collision(&self.current_piece) {
            match dir {
                Direction::Left => self.current_piece.move_dir(Direction::Right),
                Direction::Right => self.current_piece.move_dir(Direction::Left),
                Direction::Up => self.current_piece.move_dir(Direction::Down),
                Direction::Down => self.current_piece.move_dir(Direction::Up),
            }
            Err(())
        } else {
            Ok(())
        }
    }

    /// Moves the piece down by one row, freezing it if it hits an obstacle.
    fn try_move_down(&mut self) {
        self.current_piece.move_dir(Direction::Down);
        if self.lattice.check_collision(&self.current_piece) {
            self.current_piece.move_dir(Direction::Up);
            self.freeze_current_piece();
        }
    }

    /// Instantly drops the piece to its lowest valid position and locks it into the lattice.
    fn hard_drop(&mut self) {
        loop {
            self.current_piece.move_dir(Direction::Down);
            if self.lattice.check_collision(&self.current_piece) {
                self.current_piece.move_dir(Direction::Up);
                self.freeze_current_piece();
                return;
            }
        }
    }

    /// Performs a rotation on the current piece and resolves any resulting collisions.
    ///
    /// After rotating the piece's internal layout, it calls `fix_rotation` to
    /// push the piece away from walls or other blocks (Wall Kicks/Floor Kicks).
    fn rotate_piece(&mut self, rot: Rotation) {
        self.current_piece.rotate(rot);
        self.fix_rotation();
    }

    /// Swaps the current piece with the held piece, or spawns a new one if the hold is empty.
    fn hold(&mut self) {
        self.current_piece.reset();

        if let Some(held_piece) = self.held_piece.clone() {
            self.held_piece = Some(self.current_piece.clone());
            self.current_piece = held_piece;
        } else {
            self.held_piece = Some(self.current_piece.clone());
            self.reload();
        }
    }

    /// Transfers the current piece's tiles to the lattice and checks for line clears.
    fn freeze_current_piece(&mut self) {
        if self.lattice.check_ceil_collision(&self.current_piece) {
            self.game_over = true;
            return;
        }

        for tile in self.current_piece.tiles() {
            let _ = self.lattice.put(tile.to_grid_pos().unwrap(), self.current_piece.color);
        }

        self.score += self.lattice.clear_complete_rows();
        self.reload();
    }

    /// Spawns the next piece from the queue and updates the preview.
    ///
    /// This method moves the `next_piece` into the `current_piece` slot and
    /// selects a new random piece from the pool to take its place.
    ///
    /// If the newly spawned piece immediately collides with the existing
    /// lattice, the `game_over` flag is set to true.
    fn reload(&mut self) {
        self.current_piece = self.next_piece.clone();
        self.next_piece = self.piece_pool.choose(&mut self.rng).cloned().unwrap();

        if self.lattice.check_collision(&self.current_piece) {
            self.game_over = true;
        }
    }

    /// Handles "Wall Kicks" and basic floor kicks to prevent pieces from
    /// rotating into invalid positions.
    ///
    /// Note: This is a simplified rotation resolution system.
    fn fix_rotation(&mut self) {
        while self.lattice.check_left_wall_collision(&self.current_piece) {
            self.current_piece.move_dir(Direction::Right);
        }
        while self.lattice.check_right_wall_collision(&self.current_piece) {
            self.current_piece.move_dir(Direction::Left);
        }
        while self.lattice.check_tile_collision(&self.current_piece) {
            self.current_piece.move_dir(Direction::Up);
        }
    }
}
