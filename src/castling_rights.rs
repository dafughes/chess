use crate::color::Color;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CastlingRights {
    Kingside(Color) = 1,
    Queenside(Color) = 2,
}

impl CastlingRights {
    pub(crate) fn bitmask(&self) -> u8 {
        match self {
            CastlingRights::Kingside(Color::White) => 1,
            CastlingRights::Queenside(Color::White) => 2,
            CastlingRights::Kingside(Color::Black) => 4,
            CastlingRights::Queenside(Color::Black) => 8,
        }
    }
}
