use std::ops;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::bitboard::Direction;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl Rank {
    pub(crate) fn from_index(index: usize) -> Self {
        match index {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eighth,
            _ => unreachable!(),
        }
    }

    pub(crate) const fn to_index(&self) -> usize {
        *self as usize
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub(crate) fn from_index(index: usize) -> Self {
        match index {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!(),
        }
    }

    pub(crate) const fn to_index(&self) -> usize {
        *self as usize
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square(u8);

#[wasm_bindgen]
impl Square {
    /// # Example
    /// ```
    /// # use chess::square::{Rank, File, Square};
    /// assert_eq!(Square::new(Rank::Second, File::E), Square::E2);
    /// assert_eq!(Square::new(Rank::Fifth, File::C), Square::C5);
    /// assert_eq!(Square::new(Rank::Sixth, File::B), Square::B6);
    /// assert_eq!(Square::new(Rank::Second, File::F), Square::F2);
    /// assert_eq!(Square::new(Rank::First, File::A), Square::A1);
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(rank: Rank, file: File) -> Self {
        Self(((rank.to_index() << 3) + file.to_index()) as u8)
    }

    /// # Example
    /// ```
    /// # use chess::square::{Rank, File, Square};
    /// assert_eq!(Square::F8.rank(), Rank::Eighth);
    /// assert_eq!(Square::C4.rank(), Rank::Fourth);
    /// assert_eq!(Square::D6.rank(), Rank::Sixth);
    /// assert_eq!(Square::B7.rank(), Rank::Seventh);
    /// assert_eq!(Square::A3.rank(), Rank::Third);
    /// assert_eq!(Square::E4.rank(), Rank::Fourth);
    /// assert_eq!(Square::G8.rank(), Rank::Eighth);
    /// ```
    pub fn rank(&self) -> Rank {
        Rank::from_index(self.0 as usize >> 3)
    }

    /// # Example
    /// ```
    /// # use chess::square::{Rank, File, Square};
    /// assert_eq!(Square::F8.file(), File::F);
    /// assert_eq!(Square::C4.file(), File::C);
    /// assert_eq!(Square::D6.file(), File::D);
    /// assert_eq!(Square::B7.file(), File::B);
    /// assert_eq!(Square::A3.file(), File::A);
    /// assert_eq!(Square::E4.file(), File::E);
    /// assert_eq!(Square::G8.file(), File::G);
    /// ```
    pub fn file(&self) -> File {
        File::from_index(self.0 as usize & 7)
    }

    pub(crate) const fn from_index_const(index: usize) -> Self {
        Self(index as u8)
    }

    pub fn from_index(index: usize) -> Self {
        Self(index as u8)
    }

    pub(crate) const fn to_index_const(&self) -> usize {
        self.0 as usize
    }

    pub fn to_index(&self) -> usize {
        self.0 as usize
    }

    /*pub const A1: Square = Square(0);
    pub const B1: Square = Square(1);
    pub const C1: Square = Square(2);
    pub const D1: Square = Square(3);
    pub const E1: Square = Square(4);
    pub const F1: Square = Square(5);
    pub const G1: Square = Square(6);
    pub const H1: Square = Square(7);
    pub const A2: Square = Square(8);
    pub const B2: Square = Square(9);
    pub const C2: Square = Square(10);
    pub const D2: Square = Square(11);
    pub const E2: Square = Square(12);
    pub const F2: Square = Square(13);
    pub const G2: Square = Square(14);
    pub const H2: Square = Square(15);
    pub const A3: Square = Square(16);
    pub const B3: Square = Square(17);
    pub const C3: Square = Square(18);
    pub const D3: Square = Square(19);
    pub const E3: Square = Square(20);
    pub const F3: Square = Square(21);
    pub const G3: Square = Square(22);
    pub const H3: Square = Square(23);
    pub const A4: Square = Square(24);
    pub const B4: Square = Square(25);
    pub const C4: Square = Square(26);
    pub const D4: Square = Square(27);
    pub const E4: Square = Square(28);
    pub const F4: Square = Square(29);
    pub const G4: Square = Square(30);
    pub const H4: Square = Square(31);
    pub const A5: Square = Square(32);
    pub const B5: Square = Square(33);
    pub const C5: Square = Square(34);
    pub const D5: Square = Square(35);
    pub const E5: Square = Square(36);
    pub const F5: Square = Square(37);
    pub const G5: Square = Square(38);
    pub const H5: Square = Square(39);
    pub const A6: Square = Square(40);
    pub const B6: Square = Square(41);
    pub const C6: Square = Square(42);
    pub const D6: Square = Square(43);
    pub const E6: Square = Square(44);
    pub const F6: Square = Square(45);
    pub const G6: Square = Square(46);
    pub const H6: Square = Square(47);
    pub const A7: Square = Square(48);
    pub const B7: Square = Square(49);
    pub const C7: Square = Square(50);
    pub const D7: Square = Square(51);
    pub const E7: Square = Square(52);
    pub const F7: Square = Square(53);
    pub const G7: Square = Square(54);
    pub const H7: Square = Square(55);
    pub const A8: Square = Square(56);
    pub const B8: Square = Square(57);
    pub const C8: Square = Square(58);
    pub const D8: Square = Square(59);
    pub const E8: Square = Square(60);
    pub const F8: Square = Square(61);
    pub const G8: Square = Square(62);
    pub const H8: Square = Square(63);*/
}

impl ops::Add<Direction> for Square {
    type Output = Square;

    fn add(self, rhs: Direction) -> Self::Output {
        Square::from_index((self.to_index() as isize + rhs.to_index()) as usize)
    }
}

impl ops::Sub<Direction> for Square {
    type Output = Square;

    fn sub(self, rhs: Direction) -> Self::Output {
        Square::from_index((self.to_index() as isize - rhs.to_index()) as usize)
    }
}

impl From<Rank> for char {
    fn from(value: Rank) -> Self {
        ('1' as u8 + value.to_index() as u8) as char
    }
}

impl TryFrom<char> for Rank {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '1'..='8' => Ok(Rank::from_index((value as u8 - '1' as u8) as usize)),
            _ => Err(format!("Invalid rank '{}'", value)),
        }
    }
}

impl From<File> for char {
    fn from(value: File) -> Self {
        ('a' as u8 + value.to_index() as u8) as char
    }
}

impl TryFrom<char> for File {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a'..='h' => Ok(File::from_index((value as u8 - 'a' as u8) as usize)),
            _ => Err(format!("Invalid file '{}'", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_to_index() {
        assert_eq!(Rank::First.to_index(), 0);
        assert_eq!(Rank::Second.to_index(), 1);
        assert_eq!(Rank::Third.to_index(), 2);
        assert_eq!(Rank::Fourth.to_index(), 3);
        assert_eq!(Rank::Fifth.to_index(), 4);
        assert_eq!(Rank::Sixth.to_index(), 5);
        assert_eq!(Rank::Seventh.to_index(), 6);
        assert_eq!(Rank::Eighth.to_index(), 7);
    }

    #[test]
    #[should_panic]
    fn invalid_rank() {
        Rank::from_index(8);
        Rank::from_index(9);
    }

    #[test]
    fn file_to_index() {
        assert_eq!(File::A.to_index(), 0);
        assert_eq!(File::B.to_index(), 1);
        assert_eq!(File::C.to_index(), 2);
        assert_eq!(File::D.to_index(), 3);
        assert_eq!(File::E.to_index(), 4);
        assert_eq!(File::F.to_index(), 5);
        assert_eq!(File::G.to_index(), 6);
        assert_eq!(File::H.to_index(), 7);
    }

    #[test]
    #[should_panic]
    fn invalid_file() {
        File::from_index(8234);
        File::from_index(9);
    }

    #[test]
    fn square_to_index() {
        assert_eq!(Square::new(Rank::Fourth, File::E).to_index(), 28);
        assert_eq!(Square::new(Rank::Sixth, File::H).to_index(), 47);
        assert_eq!(Square::new(Rank::Second, File::B).to_index(), 9);
        assert_eq!(Square::new(Rank::Fifth, File::A).to_index(), 32);
        assert_eq!(Square::new(Rank::Third, File::C).to_index(), 18);
        assert_eq!(Square::new(Rank::First, File::C).to_index(), 2);
        assert_eq!(Square::new(Rank::Fourth, File::D).to_index(), 27);
        assert_eq!(Square::new(Rank::Eighth, File::F).to_index(), 61);
    }
}
