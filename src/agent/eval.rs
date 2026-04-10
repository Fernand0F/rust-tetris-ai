use itertools::Itertools;
use crate::engine::core::TetrisEngine;

/// Calculates the number of "holes" in the lattice.
///
/// A hole is defined as an empty cell that has at least one occupied cell
/// somewhere above it in the same column.
pub fn count_holes(engine: &TetrisEngine) -> u32 {
    let (w, h) = engine.dimensions;
    let mut hole_count = 0;

    for x in 0..w {
        let mut block_above = false;

        for y in 0..h {
            let is_filled = engine.lattice.is_occupied(x, y);

            if is_filled {
                block_above = true;
            } else if block_above {
                hole_count += 1;
            }
        }
    }

    hole_count
}

/// Calculates the aggregate height of all columns.
///
/// This is the sum of the heights of each individual column.
pub fn compute_aggregate_height(columns: &Vec<u32>) -> u32 {
    columns.iter().sum()
}

/// Calculates the "bumpiness" of the board surface.
///
/// Bumpiness is the sum of absolute differences in height between adjacent columns.
pub fn compute_bumpiness(columns: &Vec<u32>) -> u32 {
    columns.iter()
        .tuple_windows()
        .map(|(&a, &b)| a.abs_diff(b))
        .sum()
}

/// Retrieves the height of each column in the current board state.
pub fn get_column_heights(engine: &TetrisEngine) -> Vec<u32> {
    let (w, h) = engine.dimensions;

    (0..w)
        .map(|x| {
            (0..h)
                .find(|&y| engine.lattice.is_occupied(x, y))
                .map(|y| (h - y) as u32)
                .unwrap_or(0)
        })
        .collect()
}
