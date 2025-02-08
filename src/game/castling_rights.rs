use super::color::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Castling {
    Kingside(Color),
    Queenside(Color),
}

impl Castling {
    pub(crate) fn to_u8(self) -> u8 {
        match self {
            Castling::Kingside(Color::White) => 1,
            Castling::Queenside(Color::White) => 2,
            Castling::Kingside(Color::Black) => 4,
            Castling::Queenside(Color::Black) => 8,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CastlingRights(u8);

impl CastlingRights {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn add(&mut self, castling: Castling) {
        self.0 |= castling.to_u8();
    }

    pub fn remove(&mut self, castling: Castling) {
        self.0 &= !castling.to_u8();
    }

    pub fn contains(self, castling: Castling) -> bool {
        (self.0 & castling.to_u8()) != 0
    }

    pub(crate) fn to_u8(self) -> u8 {
        self.0
    }
}
