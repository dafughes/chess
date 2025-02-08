use std::cmp::Ordering;

use super::{piece::PieceKind, square::Square};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveKind {
    Quiet,
    Capture,
    DoublePush,
    EnPassant,
    CastleKingside,
    CastleQueenside,
    Promotion(PieceKind),
    PromotionCapture(PieceKind),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    from: Square,
    to: Square,
    kind: MoveKind,
}

impl Default for Move {
    fn default() -> Self {
        Self {
            from: Square::A1,
            to: Square::A1,
            kind: MoveKind::Quiet,
        }
    }
}

impl Move {
    pub fn new(from: Square, to: Square, kind: MoveKind) -> Self {
        Self { from, to, kind }
    }

    pub fn from(&self) -> Square {
        self.from
    }

    pub fn to(&self) -> Square {
        self.to
    }

    pub fn kind(&self) -> MoveKind {
        self.kind
    }

    pub fn is_null(&self) -> bool {
        self.from() == Square::A1 && self.to() == Square::A1
    }

    pub fn is_cap(&self) -> bool {
        match self.kind() {
            MoveKind::EnPassant | MoveKind::Capture | MoveKind::PromotionCapture(_) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let from_file = (b'a' + self.from().file() as u8) as char;
        let from_rank = (b'1' + self.from().rank() as u8) as char;
        let to_file = (b'a' + self.to().file() as u8) as char;
        let to_rank = (b'1' + self.to().rank() as u8) as char;

        match self.kind() {
            MoveKind::Promotion(kind) | MoveKind::PromotionCapture(kind) => {
                write!(
                    f,
                    "{}{}{}{}{}",
                    from_file,
                    from_rank,
                    to_file,
                    to_rank,
                    char::from(kind)
                )
            }
            _ => write!(f, "{}{}{}{}", from_file, from_rank, to_file, to_rank),
        }
    }
}

pub struct Movelist {
    moves: [Move; 256],
    len: usize,
}

impl Movelist {
    pub fn new() -> Self {
        Self {
            moves: [Move::default(); 256],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub(crate) fn push(&mut self, mv: Move) {
        self.moves[self.len] = mv;
        self.len += 1;
    }

    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&Move, &Move) -> Ordering,
    {
        self.moves[0..self.len].sort_by(compare);
    }

    pub fn filter<F>(&mut self, mut pred: F)
    where
        F: FnMut(&Move) -> bool,
    {
        let mut new_len = 0;
        for i in 0..self.len {
            if pred(&self.moves[i]) {
                self.moves[new_len] = self.moves[i];
                new_len += 1;
            }
        }

        self.len = new_len;
    }
}

pub struct MovelistIntoIterator<'a> {
    movelist: &'a Movelist,
    i: usize,
}

impl<'a> Iterator for MovelistIntoIterator<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.movelist.len() {
            let mv = self.movelist.moves[self.i];
            self.i += 1;

            Some(mv)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Movelist {
    type Item = Move;
    type IntoIter = MovelistIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MovelistIntoIterator {
            movelist: self,
            i: 0,
        }
    }
}

pub struct MovelistIterator {
    movelist: Movelist,
    i: usize,
}

impl Iterator for MovelistIterator {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.movelist.len() {
            let mv = self.movelist.moves[self.i];
            self.i += 1;

            Some(mv)
        } else {
            None
        }
    }
}

impl IntoIterator for Movelist {
    type Item = Move;
    type IntoIter = MovelistIterator;

    fn into_iter(self) -> Self::IntoIter {
        MovelistIterator {
            movelist: self,
            i: 0,
        }
    }
}
