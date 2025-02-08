use super::{
    color::Color,
    square::{Direction, File, Rank, Square},
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const EMPTY: Self = Self(0);

    pub fn new() -> Self {
        Self(0)
    }

    /// # Examples
    ///
    /// ```
    /// use chess::game::bitboard::Bitboard;
    ///
    /// let bb = Bitboard::EMPTY;
    /// assert!(bb.is_empty());
    /// ```
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// # Examples
    ///
    /// ```
    /// use chess::game::bitboard::Bitboard;
    ///
    /// let bb = Bitboard::EMPTY;
    /// assert!(!bb.is_non_empty());
    /// ```
    pub fn is_non_empty(self) -> bool {
        !self.is_empty()
    }

    /// Returns true if `other` is a subset of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::game::bitboard::Bitboard;
    /// use chess::game::square::{Square, Rank};
    ///
    /// assert!(Bitboard::from(Rank::Sixth).contains(Bitboard::from(Square::B6)));
    /// ```
    pub fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Returns the number of squares set in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::game::bitboard::Bitboard;
    /// use chess::game::square::Square;
    ///
    /// assert_eq!(Bitboard::EMPTY.popcount(), 0);
    /// assert_eq!(Bitboard::from(Square::E4).popcount(), 1);
    /// ```
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

    /// Shifts every bit in `self` one step in `direction`. Shifts that would result in wrapping around are masked out.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::game::bitboard::Bitboard;
    /// use chess::game::square::{Square, Direction};
    ///
    /// assert_eq!(Bitboard::from(Square::E4).shift(Direction::SW), Bitboard::from(Square::D3));
    /// assert_eq!(Bitboard::from(Square::H6).shift(Direction::NE), Bitboard::EMPTY);
    /// ```
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

    /// Starting from square `from`, goes in a straight line in `direction` while inside the board and not blocked by bits in `occupied`.
    /// Returns bits up to and including encountered occupied square but excluding `from`.
    /// # Examples
    ///
    /// ```
    /// use chess::game::bitboard::Bitboard;
    /// use chess::game::square::{Square, Direction};
    ///
    /// assert_eq!(Bitboard::ray(Square::E4, Direction::SW, Bitboard::EMPTY), Bitboard::from(Square::D3) | Bitboard::from(Square::C2) | Bitboard::from(Square::B1));
    /// ```
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

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "   | a | b | c | d | e | f | g | h |")?;
        writeln!(f, "   +---+---+---+---+---+---+---+---+---")?;

        for rank in Rank::iter().rev() {
            write!(f, " {} |", rank as usize + 1)?;
            for file in File::iter() {
                let square = Square::new(rank, file);

                if self.contains(Bitboard::from(square)) {
                    write!(f, " X |")?;
                } else {
                    write!(f, "   |")?;
                }
            }
            writeln!(f, " {}", rank as usize + 1)?;
            writeln!(f, "   +---+---+---+---+---+---+---+---+---")?;
        }
        writeln!(f, "   | a | b | c | d | e | f | g | h |")
    }
}
