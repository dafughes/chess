use std::ops;

use crate::bitboard::Direction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl ops::Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Color {
    pub(crate) fn up(&self) -> Direction {
        match self {
            Color::White => Direction::N,
            Color::Black => Direction::S,
        }
    }
}
