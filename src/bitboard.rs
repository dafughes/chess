use std::ops;

use crate::{
    color::Color,
    square::{File, Rank, Square},
};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    N,
    S,
    E,
    W,
    NE,
    SE,
    SW,
    NW,
}

impl Direction {
    pub(crate) fn to_index(&self) -> isize {
        match self {
            Direction::N => 8,
            Direction::NE => 9,
            Direction::E => 1,
            Direction::SE => -7,
            Direction::S => -8,
            Direction::SW => -9,
            Direction::W => -1,
            Direction::NW => 7,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bitboard(u64);

impl Bitboard {
    #[inline(always)]
    pub const fn new(square: Square) -> Self {
        Self(1 << square.to_index_const())
    }

    #[inline(always)]
    pub(crate) fn from_u64(value: u64) -> Self {
        Self(value)
    }

    #[inline(always)]
    pub(crate) fn to_u64(self) -> u64 {
        self.0
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    pub const fn is_non_empty(&self) -> bool {
        self.0 != 0
    }

    #[inline(always)]
    pub fn popcount(&self) -> usize {
        self.0.count_ones() as usize
    }

    #[inline(always)]
    pub fn first(&self) -> Option<Square> {
        if self.0 == 0 {
            None
        } else {
            let index = self.0.trailing_zeros();
            Some(Square::from_index_const(index as usize))
        }
    }

    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard(!0);

    pub const fn shift(&self, direction: Direction) -> Self {
        const NOT_A: u64 = !0x0101010101010101;
        const NOT_H: u64 = !0x8080808080808080;

        match direction {
            Direction::N => Self(self.0 << 8),
            Direction::S => Self(self.0 >> 8),
            Direction::E => Self((self.0 & NOT_H) << 1),
            Direction::NE => Self((self.0 & NOT_H) << 9),
            Direction::SE => Self((self.0 & NOT_H) >> 7),
            Direction::W => Self((self.0 & NOT_A) >> 1),
            Direction::NW => Self((self.0 & NOT_A) << 7),
            Direction::SW => Self((self.0 & NOT_A) >> 9),
        }
    }

    pub const fn ray(from: Square, dir: Direction, blockers: Self) -> Self {
        let mut bb = Bitboard::new(from).shift(dir);
        let mut result = Bitboard::EMPTY;

        while bb.is_non_empty() && (result.0 & blockers.0) == 0 {
            result.0 |= bb.0;
            bb = bb.shift(dir);
        }

        result
    }

    pub fn pawn_attacks(pieces: Bitboard, color: Color) -> Self {
        match color {
            Color::White => pieces.shift(Direction::NE) | pieces.shift(Direction::NW),
            Color::Black => pieces.shift(Direction::SE) | pieces.shift(Direction::SW),
        }
    }

    #[inline(always)]
    pub fn knight_attacks(from: Square) -> Self {
        KNIGHT_ATTACK_LOOKUP[from.to_index()]
        // let bb = Self::new(from);

        // bb.shift(Direction::N).shift(Direction::NE)
        //     | bb.shift(Direction::E).shift(Direction::NE)
        //     | bb.shift(Direction::E).shift(Direction::SE)
        //     | bb.shift(Direction::S).shift(Direction::SE)
        //     | bb.shift(Direction::S).shift(Direction::SW)
        //     | bb.shift(Direction::W).shift(Direction::SW)
        //     | bb.shift(Direction::W).shift(Direction::NW)
        //     | bb.shift(Direction::N).shift(Direction::NW)
    }

    #[inline(always)]
    pub fn king_attacks(from: Square) -> Self {
        KING_ATTACK_LOOKUP[from.to_index()]
        // let bb = Self::new(from);

        // bb.shift(Direction::N)
        //     | bb.shift(Direction::NE)
        //     | bb.shift(Direction::E)
        //     | bb.shift(Direction::SE)
        //     | bb.shift(Direction::S)
        //     | bb.shift(Direction::SW)
        //     | bb.shift(Direction::W)
        //     | bb.shift(Direction::NW)
    }

    pub fn bishop_attacks(from: Square, blockers: Self) -> Self {
        Self::ray(from, Direction::NE, blockers)
            | Self::ray(from, Direction::SE, blockers)
            | Self::ray(from, Direction::SW, blockers)
            | Self::ray(from, Direction::NW, blockers)
    }

    pub fn rook_attacks(from: Square, blockers: Self) -> Self {
        Self::ray(from, Direction::N, blockers)
            | Self::ray(from, Direction::E, blockers)
            | Self::ray(from, Direction::S, blockers)
            | Self::ray(from, Direction::W, blockers)
    }

    #[inline(always)]
    pub fn rank(rank: Rank) -> Self {
        Bitboard(0xFF << (rank.to_index() * 8))
    }

    #[inline(always)]
    pub fn file(file: File) -> Self {
        Bitboard(0x0101010101010101 << file.to_index())
    }

    #[inline(always)]
    pub fn rook_mask(square: Square) -> Bitboard {
        Self::rank(square.rank()) ^ Self::file(square.file())
    }

    #[inline(always)]
    pub fn bishop_mask(square: Square) -> Bitboard {
        BISHOP_MASK_LOOKUP[square.to_index()]
        // Self::bishop_attacks(square, Bitboard::EMPTY)
    }

    #[inline(always)]
    pub fn between(square1: Square, square2: Square) -> Bitboard {
        BETWEEN_LOOKUP[square1.to_index() * 64 + square2.to_index()]
        // const DIRS: [Direction; 8] = [
        //     Direction::N,
        //     Direction::NE,
        //     Direction::E,
        //     Direction::SE,
        //     Direction::S,
        //     Direction::SW,
        //     Direction::W,
        //     Direction::NW,
        // ];

        // let bb = Bitboard::new(square2);
        // for dir in DIRS {
        //     let b = Self::ray(square1, dir, bb);
        //     if (b & bb).is_non_empty() {
        //         return b ^ bb;
        //     }
        // }
        // Bitboard::EMPTY
    }
}

impl ops::BitAnd for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl ops::BitOr for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ops::BitXor for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl ops::BitAnd<Square> for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Square) -> Self::Output {
        Self(self.0 & (1 << rhs.to_index()))
    }
}

impl ops::BitOr<Square> for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Square) -> Self::Output {
        Self(self.0 | (1 << rhs.to_index()))
    }
}

impl ops::BitXor<Square> for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Square) -> Self::Output {
        Self(self.0 ^ (1 << rhs.to_index()))
    }
}

impl ops::BitAndAssign for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl ops::BitOrAssign for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl ops::BitXorAssign for Bitboard {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl ops::BitAndAssign<Square> for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Square) {
        self.0 &= 1 << rhs.to_index();
    }
}

impl ops::BitOrAssign<Square> for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Square) {
        self.0 |= 1 << rhs.to_index();
    }
}

impl ops::BitXorAssign<Square> for Bitboard {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Square) {
        self.0 ^= 1 << rhs.to_index();
    }
}

impl ops::Not for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = BitboardIntoIterator;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        BitboardIntoIterator(self.0)
    }
}

pub struct BitboardIntoIterator(u64);

impl Iterator for BitboardIntoIterator {
    type Item = Square;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let index = self.0.trailing_zeros() as usize;

            let square: Square = Square::from_index_const(index);
            self.0 &= self.0.wrapping_sub(1);
            Some(square)
        }
    }
}

static BETWEEN_LOOKUP: [Bitboard; 64 * 64] = init_between_lookup();
static BISHOP_MASK_LOOKUP: [Bitboard; 64] = init_bishop_mask_lookup();
static KNIGHT_ATTACK_LOOKUP: [Bitboard; 64] = init_knight_attack_lookup();
static KING_ATTACK_LOOKUP: [Bitboard; 64] = init_king_attack_lookup();

const fn init_king_attack_lookup() -> [Bitboard; 64] {
    let mut lookup = [Bitboard::EMPTY; 64];

    let mut i = 0;
    while i < 64 {
        let from = Square::from_index_const(i);

        let bb = Bitboard::new(from);

        let x = bb.shift(Direction::N).0
            | bb.shift(Direction::NE).0
            | bb.shift(Direction::E).0
            | bb.shift(Direction::SE).0
            | bb.shift(Direction::S).0
            | bb.shift(Direction::SW).0
            | bb.shift(Direction::W).0
            | bb.shift(Direction::NW).0;

        lookup[i] = Bitboard(x);

        i += 1;
    }

    lookup
}

const fn init_knight_attack_lookup() -> [Bitboard; 64] {
    let mut lookup = [Bitboard::EMPTY; 64];

    let mut i = 0;
    while i < 64 {
        let from = Square::from_index_const(i);

        let bb = Bitboard::new(from);

        let x = bb.shift(Direction::N).shift(Direction::NE).0
            | bb.shift(Direction::E).shift(Direction::NE).0
            | bb.shift(Direction::E).shift(Direction::SE).0
            | bb.shift(Direction::S).shift(Direction::SE).0
            | bb.shift(Direction::S).shift(Direction::SW).0
            | bb.shift(Direction::W).shift(Direction::SW).0
            | bb.shift(Direction::W).shift(Direction::NW).0
            | bb.shift(Direction::N).shift(Direction::NW).0;

        lookup[i] = Bitboard(x);

        i += 1;
    }

    lookup
}

const fn init_bishop_mask_lookup() -> [Bitboard; 64] {
    const DIRECTIONS: [Direction; 4] = [Direction::NE, Direction::SE, Direction::SW, Direction::NW];

    let mut lookup = [Bitboard::EMPTY; 64];

    let mut square = 0;
    while square < 64 {
        let mut i = 0;
        let mut b: u64 = 0;
        while i < 4 {
            let d = DIRECTIONS[i];
            let r = Bitboard::ray(Square::from_index_const(square), d, Bitboard::EMPTY);
            b |= r.0;

            i += 1;
        }

        lookup[square] = Bitboard(b);

        square += 1;
    }
    lookup
}

const fn init_between_lookup() -> [Bitboard; 64 * 64] {
    let mut lookup = [Bitboard::EMPTY; 64 * 64];

    const DIRECTIONS: [Direction; 8] = [
        Direction::N,
        Direction::S,
        Direction::E,
        Direction::W,
        Direction::NE,
        Direction::SE,
        Direction::SW,
        Direction::NW,
    ];

    let mut square1 = 0;
    while square1 < 64 {
        let mut square2 = 0;
        while square2 < 64 {
            let bb2 = Bitboard::new(Square::from_index_const(square2));

            let mut i = 0;
            while i < 8 {
                let d = DIRECTIONS[i];
                let r = Bitboard::ray(Square::from_index_const(square1), d, bb2);

                if (r.0 & bb2.0) != 0 {
                    let square_index = square1 * 64 + square2;
                    lookup[square_index] = Bitboard(r.0 ^ bb2.0);
                    break;
                }

                i += 1;
            }

            square2 += 1;
        }

        square1 += 1;
    }

    lookup
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitboard_popcount() {
        assert_eq!(Bitboard(255).popcount(), 8);
    }

    #[test]
    fn bitboard_iterator() {
        let mut it = Bitboard(255).into_iter();

        assert_eq!(it.next(), Some(Square::new(Rank::First, File::A)));
        assert_eq!(it.next(), Some(Square::new(Rank::First, File::B)));
        assert_eq!(it.next(), Some(Square::new(Rank::First, File::C)));
        assert_eq!(it.next(), Some(Square::new(Rank::First, File::D)));
        assert_eq!(it.next(), Some(Square::new(Rank::First, File::E)));
        assert_eq!(it.next(), Some(Square::new(Rank::First, File::F)));
        assert_eq!(it.next(), Some(Square::new(Rank::First, File::G)));
        assert_eq!(it.next(), Some(Square::new(Rank::First, File::H)));
        assert_eq!(it.next(), None);
    }
}
