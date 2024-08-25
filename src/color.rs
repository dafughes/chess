use std::ops;

use crate::bitboard::Direction;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl ops::Not for Color {
    type Output = Color;

    /// Flips the color from white to black and vice-versa.
    /// # Example
    /// ```
    /// # use chess::color::Color;
    /// assert_eq!(!Color::White, Color::Black);
    /// assert_eq!(!Color::Black, Color::White);
    /// ```
    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Color {
    pub fn up(&self) -> Direction {
        match self {
            Color::White => Direction::N,
            Color::Black => Direction::S,
        }
    }

    pub(crate) fn to_index(&self) -> usize {
        *self as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_to_index() {
        assert_eq!(Color::White.to_index(), 0);
        assert_eq!(Color::Black.to_index(), 1);
    }
}
