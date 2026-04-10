# Rust Tetris AI
A Tetris environment and automated solver written in Rust. This project features a heuristic-based AI agent that uses Lookahead Search to optimize moves in real-time.

## Key Features
Heuristic Solver: Evaluates board states using a weighted linear combination of features.

Lookahead Search: Simulates future piece placements to avoid dead-ends.

Macroquad Rendering: A lightweight, hardware-accelerated 2D renderer.

## The AI Agent
The agent acts as a "Greedy Search" algorithm. For every new piece, it simulates every possible rotation and horizontal position, scoring the resulting board:

**Aggregate Height**: Sum of all column heights.

**Holes**: Empty cells blocked by tiles above them.

**Bumpiness**: Total difference in height between adjacent columns.

## Search Strategy
The agent doesn't just look at the current piece. It recursively evaluates the Next Piece and the Held Piece up to a specific lookahead_depth. This allows it to make strategic sacrifices (like using the Hold slot) to maintain a cleaner board.

## Technical Architecture
The project follows strict Separation of Concerns.

**engine/:** The "source of truth." Manages physics, collision detection, and game rules.

**agent/:** The brain. Contains the move evaluator and the search logic.

**render.rs:** Handles drawing and UI layout.

**common/:** Geometric primitives (Vec2), Directions, and Rotations.

## AI Configuration
You can tweak the agent's "personality" in main.rs by adjusting its weights:

```
let agent = TetrisAgent {
    holes_w: -1.5,      // Higher negative value = avoids holes more aggressively
    height_w: -2.0,     // Higher negative value = keeps the board lower
    bumpiness_w: -1.0,  // Higher negative value = prefers a flat surface
    lookahead_depth: 1, // Number of future pieces to simulate
};
```
