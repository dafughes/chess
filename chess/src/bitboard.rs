use crate::board::{
    color::Color,
    square::{Direction, File, Rank, Square},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn is_non_empty(self) -> bool {
        !self.is_empty()
    }

    /// Returns true if `other` is a subset of `self`.
    pub fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    pub fn popcount(self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn pop_lsb(&mut self) -> Option<Square> {
        if self.0 == 0 {
            None
        } else {
            let i = self.0.trailing_zeros() as u8;
            let square = Square::from_u8(i);
            self.0 &= self.0.wrapping_sub(1);
            Some(square)
        }
    }

    pub fn shift(self, direction: Direction) -> Self {
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

    pub fn ray(from: Square, direction: Direction, occupied: Self) -> Self {
        let mut bb = Bitboard::from(from).shift(direction);
        let mut result = Bitboard::new();

        while bb.is_non_empty() && (result & occupied).is_empty() {
            result |= bb;
            bb = bb.shift(direction);
        }

        result
    }

    pub fn pawn_attacks(from: Square, color: Color) -> Self {
        let bb = Bitboard::from(from);
        match color {
            Color::White => bb.shift(Direction::NE) | bb.shift(Direction::NW),
            Color::Black => bb.shift(Direction::SE) | bb.shift(Direction::SW),
        }
    }

    pub fn knight_attacks(from: Square) -> Self {
        let bb = Self::from(from);
        bb.shift(Direction::N).shift(Direction::NE)
            | bb.shift(Direction::E).shift(Direction::NE)
            | bb.shift(Direction::E).shift(Direction::SE)
            | bb.shift(Direction::S).shift(Direction::SE)
            | bb.shift(Direction::S).shift(Direction::SW)
            | bb.shift(Direction::W).shift(Direction::SW)
            | bb.shift(Direction::W).shift(Direction::NW)
            | bb.shift(Direction::N).shift(Direction::NW)
    }

    pub fn king_attacks(from: Square) -> Self {
        let bb = Self::from(from);

        bb.shift(Direction::N)
            | bb.shift(Direction::NE)
            | bb.shift(Direction::E)
            | bb.shift(Direction::SE)
            | bb.shift(Direction::S)
            | bb.shift(Direction::SW)
            | bb.shift(Direction::W)
            | bb.shift(Direction::NW)
    }

    pub fn bishop_attacks(from: Square, occupied: Self) -> Self {
        Self::ray(from, Direction::NE, occupied)
            | Self::ray(from, Direction::SE, occupied)
            | Self::ray(from, Direction::SW, occupied)
            | Self::ray(from, Direction::NW, occupied)
    }

    pub fn rook_attacks(from: Square, occupied: Self) -> Self {
        Self::ray(from, Direction::N, occupied)
            | Self::ray(from, Direction::E, occupied)
            | Self::ray(from, Direction::S, occupied)
            | Self::ray(from, Direction::W, occupied)
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Self(0)
    }
}

impl From<Square> for Bitboard {
    fn from(value: Square) -> Self {
        Self(1 << value as usize)
    }
}

impl From<Rank> for Bitboard {
    fn from(value: Rank) -> Self {
        Self(0xff << (value as usize * 8))
    }
}

impl From<File> for Bitboard {
    fn from(value: File) -> Self {
        Self(0x0101010101010101 << value as usize)
    }
}

pub struct BitboardIntoIterator(u64);

impl Iterator for BitboardIntoIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let i = self.0.trailing_zeros() as u8;
            let square = Square::from_u8(i);
            self.0 &= self.0.wrapping_sub(1);
            Some(square)
        }
    }
}

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = BitboardIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitboardIntoIterator(self.0)
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        bitboard::Bitboard,
        board::square::{Rank, Square},
    };

    #[test]
    fn popcount() {
        assert_eq!(Bitboard(3).popcount(), 2);
        assert_eq!(Bitboard(255).popcount(), 8);
        assert_eq!(Bitboard::from(Rank::Seventh).popcount(), 8);
    }

    #[test]
    fn shift() {
        assert_eq!(
            Bitboard::from(Square::E4).shift(super::Direction::NW),
            Bitboard::from(Square::D5)
        );

        assert_eq!(
            Bitboard::from(Rank::First).shift(super::Direction::NE),
            Bitboard::from(Rank::Second) ^ Bitboard::from(Square::A2)
        );
    }

    #[test]
    fn ray() {
        let squares: Vec<_> = Bitboard::ray(Square::E4, super::Direction::SW, Bitboard::new())
            .into_iter()
            .collect();

        assert_eq!(squares, vec![Square::B1, Square::C2, Square::D3]);
    }
}
