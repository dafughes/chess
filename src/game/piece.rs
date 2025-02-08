use super::color::Color;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceKind {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl Piece {
    /// ```
    /// use chess::game::{color::Color, piece::{PieceKind, Piece}};
    /// assert_eq!(Piece::new(PieceKind::Knight, Color::Black), Piece::BlackKnight);
    /// assert_eq!(Piece::new(PieceKind::Rook, Color::White), Piece::WhiteRook);
    /// ```
    pub fn new(kind: PieceKind, color: Color) -> Self {
        match (kind, color) {
            (PieceKind::Pawn, Color::White) => Piece::WhitePawn,
            (PieceKind::Knight, Color::White) => Piece::WhiteKnight,
            (PieceKind::Bishop, Color::White) => Piece::WhiteBishop,
            (PieceKind::Rook, Color::White) => Piece::WhiteRook,
            (PieceKind::Queen, Color::White) => Piece::WhiteQueen,
            (PieceKind::King, Color::White) => Piece::WhiteKing,
            (PieceKind::Pawn, Color::Black) => Piece::BlackPawn,
            (PieceKind::Knight, Color::Black) => Piece::BlackKnight,
            (PieceKind::Bishop, Color::Black) => Piece::BlackBishop,
            (PieceKind::Rook, Color::Black) => Piece::BlackRook,
            (PieceKind::Queen, Color::Black) => Piece::BlackQueen,
            (PieceKind::King, Color::Black) => Piece::BlackKing,
        }
    }

    /// ```
    /// use chess::game::piece::{PieceKind, Piece};
    /// assert_eq!(Piece::BlackQueen.kind(), PieceKind::Queen);
    /// assert_eq!(Piece::WhitePawn.kind(), PieceKind::Pawn);
    /// assert_eq!(Piece::BlackBishop.kind(), PieceKind::Bishop);
    /// ```
    pub fn kind(&self) -> PieceKind {
        match self {
            Piece::WhitePawn | Piece::BlackPawn => PieceKind::Pawn,
            Piece::WhiteKnight | Piece::BlackKnight => PieceKind::Knight,
            Piece::WhiteBishop | Piece::BlackBishop => PieceKind::Bishop,
            Piece::WhiteRook | Piece::BlackRook => PieceKind::Rook,
            Piece::WhiteQueen | Piece::BlackQueen => PieceKind::Queen,
            Piece::WhiteKing | Piece::BlackKing => PieceKind::King,
        }
    }

    /// ```
    /// use chess::game::{color::Color, piece::Piece};
    /// assert_eq!(Piece::BlackQueen.color(), Color::Black);
    /// assert_eq!(Piece::WhitePawn.color(), Color::White);
    /// assert_eq!(Piece::BlackBishop.color(), Color::Black);
    /// ```
    pub fn color(&self) -> Color {
        match self {
            Piece::WhitePawn
            | Piece::WhiteKnight
            | Piece::WhiteBishop
            | Piece::WhiteRook
            | Piece::WhiteQueen
            | Piece::WhiteKing => Color::White,
            _ => Color::Black,
        }
    }
}

impl TryFrom<char> for PieceKind {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'p' => Ok(PieceKind::Pawn),
            'n' => Ok(PieceKind::Knight),
            'b' => Ok(PieceKind::Bishop),
            'r' => Ok(PieceKind::Rook),
            'q' => Ok(PieceKind::Queen),
            'k' => Ok(PieceKind::King),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'P' => Ok(Piece::WhitePawn),
            'N' => Ok(Piece::WhiteKnight),
            'B' => Ok(Piece::WhiteBishop),
            'R' => Ok(Piece::WhiteRook),
            'Q' => Ok(Piece::WhiteQueen),
            'K' => Ok(Piece::WhiteKing),
            'p' => Ok(Piece::BlackPawn),
            'n' => Ok(Piece::BlackKnight),
            'b' => Ok(Piece::BlackBishop),
            'r' => Ok(Piece::BlackRook),
            'q' => Ok(Piece::BlackQueen),
            'k' => Ok(Piece::BlackKing),
            _ => Err(()),
        }
    }
}

impl From<Piece> for char {
    fn from(value: Piece) -> Self {
        match value {
            Piece::WhitePawn => 'P',
            Piece::WhiteKnight => 'N',
            Piece::WhiteBishop => 'B',
            Piece::WhiteRook => 'R',
            Piece::WhiteQueen => 'Q',
            Piece::WhiteKing => 'K',
            Piece::BlackPawn => 'p',
            Piece::BlackKnight => 'n',
            Piece::BlackBishop => 'b',
            Piece::BlackRook => 'r',
            Piece::BlackQueen => 'q',
            Piece::BlackKing => 'k',
        }
    }
}

impl From<PieceKind> for char {
    fn from(value: PieceKind) -> Self {
        match value {
            PieceKind::Pawn => 'p',
            PieceKind::Knight => 'n',
            PieceKind::Bishop => 'b',
            PieceKind::Rook => 'r',
            PieceKind::Queen => 'q',
            PieceKind::King => 'k',
        }
    }
}
