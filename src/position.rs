//! Implementation of a 2D position tracker with inteager x, y and binary operations

use std::ops::{Add, AddAssign, Sub};
// Errors
#[derive(Debug)]
pub struct IntConversionError {
    convertion_attempt: (i32,i32),
}
impl std::fmt::Display for IntConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl std::error::Error for IntConversionError {}

//
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Pos {
    pub x: i32,
    pub y: i32
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn sign(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    /// Returns Err if either x or y are negative.
    pub fn as_usize_tuple(&self) -> Result<(usize,usize),IntConversionError> {
        if self.x < 0 || self.y < 0 {
            Err(IntConversionError{convertion_attempt: (self.x,self.y)})
        }
        else {
            Ok((self.x as usize, self.y as usize))
        }
    }
}
impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}
impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

