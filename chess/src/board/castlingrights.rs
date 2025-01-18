use super::color::Color;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Castling {
    Kingside(Color) = 1,
    Queenside(Color) = 2,
}

impl Castling {
    fn to_u8(self) -> u8 {
        match self {
            Castling::Kingside(Color::White) => 1,
            Castling::Queenside(Color::White) => 2,
            Castling::Kingside(Color::Black) => 4,
            Castling::Queenside(Color::Black) => 8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CastlingRights(u8);

impl CastlingRights {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn contains(self, c: Castling) -> bool {
        (self.0 & c.to_u8()) != 0
    }

    pub fn add(&mut self, c: Castling) {
        self.0 |= c.to_u8();
    }

    pub fn remove(&mut self, c: Castling) {
        self.0 &= !c.to_u8();
    }
}

impl std::fmt::Display for CastlingRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.contains(Castling::Kingside(Color::White)) {
            write!(f, "K")?;
        }
        if self.contains(Castling::Queenside(Color::White)) {
            write!(f, "Q")?;
        }
        if self.contains(Castling::Kingside(Color::Black)) {
            write!(f, "k")?;
        }
        if self.contains(Castling::Queenside(Color::Black)) {
            write!(f, "q")?;
        }
        if self.0 == 0 {
            write!(f, "-")?;
        }

        write!(f, "")
    }
}

impl std::str::FromStr for CastlingRights {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cr = CastlingRights::new();

        if s == "-" {
            Ok(cr)
        } else {
            for c in s.chars() {
                match c {
                    'K' => cr.add(Castling::Kingside(Color::White)),
                    'Q' => cr.add(Castling::Queenside(Color::White)),
                    'k' => cr.add(Castling::Kingside(Color::Black)),
                    'q' => cr.add(Castling::Queenside(Color::Black)),
                    _ => return Err(()),
                }
            }
            Ok(cr)
        }
    }
}
