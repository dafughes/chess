use std::ops;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::bitboard::Direction;

#[wasm_bindgen]
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

    // pub(crate) fn to_index(&self) -> usize {
    //     *self as usize
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn player_to_index() {
//         assert_eq!(Color::White.to_index(), 0);
//         assert_eq!(Color::Black.to_index(), 1);
//     }
// }
