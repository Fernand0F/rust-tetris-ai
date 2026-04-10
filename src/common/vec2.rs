use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};
use crate::common::rotation::Rotation;

/// A 2D vector representation used for positional math on the game board.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// Creates a new `Vec2`.
pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

impl Vec2 {
    /// Rotates the vector in-place around the origin (0, 0).
    ///
    /// # Arguments
    /// * `rotation` - The direction of the rotation.
    pub fn rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::None => {},
            Rotation::Clockwise => (self.x, self.y) = (-self.y, self.x),
            Rotation::CounterClockwise => (self.x, self.y) = (self.y, -self.x),
            Rotation::Opposite => (self.x, self.y) = (-self.x, -self.y)
        }
    }

    /// Converts the continuous vector position into a discrete grid coordinate.
    ///
    /// Returns `None` if the vector contains negative values or fractional parts,
    /// as grid positions must be exact positive integers.
    pub fn to_grid_pos(self) -> Option<(usize, usize)> {
        if self.x < 0.0 || self.y < 0.0 || self.x.fract() != 0.0 || self.y.fract() != 0.0 {
            None
        } else {
            Some((self.x as usize, self.y as usize))
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        vec2(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        vec2(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        vec2(-self.x, -self.y)
    }
}
