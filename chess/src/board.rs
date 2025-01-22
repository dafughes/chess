use castlingrights::{Castling, CastlingRights};
use color::Color;
use moves::Move;
use piece::{Piece, PieceKind};
use square::{File, Rank, Square};

use crate::bitboard::Bitboard;

pub mod castlingrights;
pub mod color;
pub mod hash;
pub mod moves;
pub mod piece;
pub mod square;

#[derive(Debug, Clone)]
pub struct Board {
    squares: [Option<Piece>; 64],
    pieces_bb: [Bitboard; 6],
    colors_bb: [Bitboard; 2],
    fullmove: usize,
    color: Color,
    cr: CastlingRights,
    ep: Option<Square>,
    halfmove: u8,
    hash: u64,
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.squares == other.squares
            && self.color == other.color
            && self.cr == other.cr
            && self.ep == other.ep
    }
}

impl Eq for Board {}

fn calculate_hash(board: &Board) -> u64 {
    let mut hash = 0;
    for rank in Rank::iter() {
        for file in File::iter() {
            let square = Square::new(rank, file);
            if let Some(piece) = board.get(square) {
                hash ^= self::hash::PIECES_HASH[square as usize][piece as usize];
            }
        }
    }

    if board.color_to_move() == Color::Black {
        hash ^= self::hash::BLACK_HASH;
    }

    hash ^= self::hash::CR_HASH[board.castling_rights().0 as usize];

    if let Some(ep) = board.en_passant_square() {
        hash ^= self::hash::EP_HASH[ep.file() as usize];
    }

    hash
}

impl Board {
    pub const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// Creates an empty chessboard.
    pub fn new() -> Self {
        let mut board = Self {
            squares: [None; 64],
            pieces_bb: [Bitboard::default(); 6],
            colors_bb: [Bitboard::default(); 2],
            fullmove: 1,
            color: Color::White,
            cr: CastlingRights::new(),
            ep: None,
            halfmove: 0,
            hash: 0,
        };

        board.hash = calculate_hash(&board);
        board
    }

    /// TODO: Explain why not implement the Hash trait.
    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub fn color_to_move(&self) -> Color {
        self.color
    }

    pub fn castling_rights(&self) -> CastlingRights {
        self.cr
    }

    pub fn en_passant_square(&self) -> Option<Square> {
        self.ep
    }

    /// Number of plies since a capture or pawn move. Used for the 50-move rule.
    pub fn halfmove_clock(&self) -> u32 {
        self.halfmove as u32
    }

    pub fn fullmove_number(&self) -> u32 {
        self.fullmove as u32
    }

    /// Returns the piece, if any on `square`.
    pub fn get(&self, square: Square) -> Option<Piece> {
        self.squares[square as usize]
    }

    /// Puts a piece on a square.
    pub fn set(&mut self, piece: Piece, square: Square) {
        self.squares[square as usize] = Some(piece);

        let bb = Bitboard::from(square);
        self.pieces_bb[piece.kind() as usize] |= bb;
        self.colors_bb[piece.color() as usize] |= bb;

        // Update hash
        self.hash ^= hash::PIECES_HASH[square as usize][piece as usize];
    }

    pub fn pieces_by_kind(&self, kind: PieceKind) -> Bitboard {
        self.pieces_bb[kind as usize]
    }

    pub fn pieces_by_color(&self, color: Color) -> Bitboard {
        self.colors_bb[color as usize]
    }

    pub fn pieces(&self) -> Bitboard {
        self.colors_bb[0] | self.colors_bb[1]
    }

    fn take_piece(&mut self, square: Square) -> Option<Piece> {
        if let Some(piece) = self.squares[square as usize].take() {
            let bb = !Bitboard::from(square);
            self.pieces_bb[piece.kind() as usize] &= bb;
            self.colors_bb[piece.color() as usize] &= bb;

            // Update hash
            self.hash ^= hash::PIECES_HASH[square as usize][piece as usize];

            Some(piece)
        } else {
            None
        }
    }

    fn move_piece(&mut self, from: Square, to: Square) {
        if let Some(piece) = self.squares[from as usize].take() {
            self.squares[to as usize] = Some(piece);
            let bb = Bitboard::from(from) ^ Bitboard::from(to);
            self.pieces_bb[piece.kind() as usize] ^= bb;
            self.colors_bb[piece.color() as usize] ^= bb;

            // Update hash
            self.hash ^= hash::PIECES_HASH[from as usize][piece as usize];
            self.hash ^= hash::PIECES_HASH[to as usize][piece as usize];
        }
    }

    pub fn do_move(&self, mv: Move) -> Self {
        let from = mv.from();
        let to = mv.to();

        let moved_piece = self.get(from).unwrap();

        let mut new_board = Self {
            halfmove: if moved_piece.kind() == PieceKind::Pawn {
                0
            } else {
                self.halfmove + 1
            },
            ep: None,
            ..*self
        };

        // Update hash: en passant
        if let Some(ep) = self.en_passant_square() {
            new_board.hash ^= hash::EP_HASH[ep.file() as usize];
        }

        match mv.kind() {
            moves::MoveKind::Quiet => new_board.move_piece(from, to),
            moves::MoveKind::Capture => {
                _ = new_board.take_piece(to);
                new_board.halfmove = 0;
                new_board.move_piece(from, to);
            }
            moves::MoveKind::DoublePush => {
                let ep = Square::from_u8((from as u8 + to as u8) / 2);
                new_board.ep = Some(ep);
                new_board.move_piece(from, to);

                new_board.hash ^= hash::EP_HASH[ep.file() as usize];
            }
            moves::MoveKind::EnPassant => {
                let capture_square = Square::new(from.rank(), to.file());
                _ = new_board.take_piece(capture_square);
                new_board.move_piece(from, to);
            }
            moves::MoveKind::CastleKingside => {
                let (rfrom, rto) = (
                    Square::from_u8(from as u8 + 3),
                    Square::from_u8(from as u8 + 1),
                );

                new_board.move_piece(from, to);
                new_board.move_piece(rfrom, rto);
            }
            moves::MoveKind::CastleQueenside => {
                let (rfrom, rto) = (
                    Square::from_u8(from as u8 - 4),
                    Square::from_u8(from as u8 - 1),
                );
                new_board.move_piece(from, to);
                new_board.move_piece(rfrom, rto);
            }
            moves::MoveKind::Promotion(piece_kind) => {
                _ = new_board.take_piece(from);
                new_board.set(Piece::new(piece_kind, new_board.color), to);
            }
            moves::MoveKind::PromotionCapture(piece_kind) => {
                _ = new_board.take_piece(from);
                _ = new_board.take_piece(to);
                new_board.set(Piece::new(piece_kind, new_board.color), to);
            }
        }

        match from {
            Square::H1 => new_board.cr.remove(Castling::Kingside(Color::White)),
            Square::A1 => new_board.cr.remove(Castling::Queenside(Color::White)),
            Square::E1 => {
                new_board.cr.remove(Castling::Kingside(Color::White));
                new_board.cr.remove(Castling::Queenside(Color::White));
            }
            Square::H8 => new_board.cr.remove(Castling::Kingside(Color::Black)),
            Square::A8 => new_board.cr.remove(Castling::Queenside(Color::Black)),
            Square::E8 => {
                new_board.cr.remove(Castling::Kingside(Color::Black));
                new_board.cr.remove(Castling::Queenside(Color::Black));
            }
            _ => (),
        }
        match to {
            Square::H1 => new_board.cr.remove(Castling::Kingside(Color::White)),
            Square::A1 => new_board.cr.remove(Castling::Queenside(Color::White)),
            Square::H8 => new_board.cr.remove(Castling::Kingside(Color::Black)),
            Square::A8 => new_board.cr.remove(Castling::Queenside(Color::Black)),
            _ => (),
        }

        // Update hash: color & cr
        new_board.hash ^= hash::CR_HASH[self.castling_rights().0 as usize];
        new_board.hash ^= hash::CR_HASH[new_board.castling_rights().0 as usize];

        new_board.hash ^= hash::BLACK_HASH;

        if new_board.color == Color::Black {
            new_board.fullmove += 1;
        }

        new_board.color = !new_board.color;

        new_board
    }

    /// Is the current player in check?
    pub fn in_check(&self) -> bool {
        self.color_in_check(self.color)
    }

    /// Is `color` in check? Used internally in legal move filtering.
    fn color_in_check(&self, color: Color) -> bool {
        let king_square = (self.pieces_by_kind(PieceKind::King) & self.pieces_by_color(color))
            .pop_lsb()
            .unwrap();

        self.is_square_attacked(!color, king_square)
    }

    fn is_square_attacked(&self, attacker: Color, square: Square) -> bool {
        let attacker_bb = self.pieces_by_color(attacker);

        let pawns = self.pieces_by_kind(PieceKind::Pawn) & attacker_bb;
        let knights = self.pieces_by_kind(PieceKind::Knight) & attacker_bb;
        let bishops = self.pieces_by_kind(PieceKind::Bishop) & attacker_bb;
        let rooks = self.pieces_by_kind(PieceKind::Rook) & attacker_bb;
        let queens = self.pieces_by_kind(PieceKind::Queen) & attacker_bb;
        let king = self.pieces_by_kind(PieceKind::King) & attacker_bb;

        let a = Bitboard::rook_attacks(square, self.pieces()) & (queens | rooks);
        let b = Bitboard::bishop_attacks(square, self.pieces()) & (queens | bishops);
        let c = Bitboard::knight_attacks(square) & knights;
        let d = Bitboard::king_attacks(square) & king;
        let e = Bitboard::pawn_attacks(square, !attacker) & pawns;

        (a | b | c | d | e).is_non_empty()
    }

    pub fn fen(&self) -> String {
        let mut result = String::new();

        // Pieces
        for rank in Rank::iter().rev() {
            let mut empty_squares = 0;
            for file in File::iter() {
                let square = Square::new(rank, file);

                if let Some(piece) = self.get(square) {
                    if empty_squares > 0 {
                        result.push((empty_squares as u8 + b'0') as char);
                        empty_squares = 0;
                    }
                    result.push(piece.into());
                } else {
                    empty_squares += 1;
                }
            }
            if empty_squares > 0 {
                result.push((empty_squares as u8 + b'0') as char);
            }
            if rank != Rank::First {
                result.push('/');
            }
        }

        result.push_str(
            format!(
                " {} {} {} {} {}",
                self.color_to_move(),
                self.castling_rights(),
                self.en_passant_square()
                    .map_or(String::from("-"), |s| s.to_string()),
                self.halfmove_clock(),
                self.fullmove_number()
            )
            .as_str(),
        );

        result
    }
}

impl Default for Board {
    /// Creates a chessboard with standard starting position.
    fn default() -> Self {
        Self::STARTPOS.parse().unwrap()
    }
}

impl std::str::FromStr for Board {
    type Err = ();

    /// Tries to create a chessboard by parsing a FEN string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board::new();
        let mut fields = s.split_ascii_whitespace();

        let mut rank = Rank::iter().rev().peekable();
        let mut file = File::iter().peekable();

        for c in fields.next().ok_or(())?.chars() {
            match c {
                '/' => {
                    if *rank.peek().ok_or(())? == Rank::First {
                        return Err(());
                    }

                    // file must be consumed
                    if file.peek().is_some() {
                        return Err(());
                    } else {
                        rank.next();
                        file = File::iter().peekable();
                    }
                }
                '1'..='8' => {
                    let n = (c as u8 - b'0') as usize;
                    (0..n).for_each(|_| _ = file.next());
                }
                _ => {
                    // file must be ok
                    let square = Square::new(*rank.peek().ok_or(())?, file.next().ok_or(())?);
                    let piece = c.try_into()?;

                    board.set(piece, square);
                }
            }
        }

        board.color = fields.next().ok_or(())?.parse()?;

        board.cr = fields.next().unwrap_or("-").parse()?;

        board.ep = match fields.next().unwrap_or("-") {
            "-" => None,
            s => Some(s.parse()?),
        };

        board.halfmove = fields.next().unwrap_or("0").parse::<u8>().map_err(|_| ())?;
        board.fullmove = fields
            .next()
            .unwrap_or("1")
            .parse::<usize>()
            .map_err(|_| ())?;

        board.hash = calculate_hash(&board);

        Ok(board)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "   | a | b | c | d | e | f | g | h |")?;
        writeln!(f, "   +---+---+---+---+---+---+---+---+---")?;

        for rank in Rank::iter().rev() {
            write!(f, " {} |", rank)?;
            for file in File::iter() {
                let c = if let Some(piece) = self.get(Square::new(rank, file)) {
                    piece.into()
                } else {
                    ' '
                };
                write!(f, " {} |", c)?;
            }
            writeln!(f, " {}", rank)?;
            writeln!(f, "   +---+---+---+---+---+---+---+---+---")?;
        }
        writeln!(
            f,
            "   | a | b | c | d | e | f | g | h |\n\nFEN: {}",
            self.fen()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::board::{color::Color, square::Square};

    use super::{
        moves::{Move, MoveKind},
        piece::Piece,
        square::{File, Rank},
        Board,
    };

    #[test]
    fn get_on_empty_board() {
        let board = Board::new();
        for rank in Rank::iter() {
            for file in File::iter() {
                assert_eq!(board.get(Square::new(rank, file)), None);
            }
        }
    }

    #[test]
    fn default_board() {
        let board = Board::default();
        File::iter().for_each(|f| {
            assert_eq!(
                board.get(Square::new(Rank::Second, f)),
                Some(Piece::WhitePawn)
            )
        });
        File::iter().for_each(|f| {
            assert_eq!(
                board.get(Square::new(Rank::Seventh, f)),
                Some(Piece::BlackPawn)
            )
        });

        assert_eq!(board.get(Square::A1), Some(Piece::WhiteRook));
        assert_eq!(board.get(Square::H1), Some(Piece::WhiteRook));
        assert_eq!(board.get(Square::D1), Some(Piece::WhiteQueen));
        assert_eq!(board.get(Square::E8), Some(Piece::BlackKing));
    }

    #[test]
    fn fen_parsing() {
        let board: Board = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"
            .parse()
            .unwrap();

        assert_eq!(board.get(Square::C4), Some(Piece::WhiteBishop));
        assert_eq!(board.get(Square::F2), Some(Piece::BlackKnight));
        assert_eq!(board.get(Square::D7), Some(Piece::WhitePawn));

        assert_eq!(board.color_to_move(), Color::White);
        assert_eq!(board.castling_rights().to_string(), String::from("KQ"));
        assert_eq!(board.en_passant_square(), None);

        assert_eq!(board.halfmove_clock(), 1);
        assert_eq!(board.fullmove_number(), 8);
    }

    #[test]
    fn board_equality() {
        let mut board1 = Board::default();
        board1 = board1.do_move(Move::new(Square::A2, Square::A4, MoveKind::DoublePush));
        board1 = board1.do_move(Move::new(Square::B8, Square::A6, MoveKind::Quiet));
        board1 = board1.do_move(Move::new(Square::E2, Square::E3, MoveKind::Quiet));
        board1 = board1.do_move(Move::new(Square::G8, Square::H6, MoveKind::Quiet));

        let mut board2 = Board::default();
        board2 = board2.do_move(Move::new(Square::A2, Square::A4, MoveKind::DoublePush));
        board2 = board2.do_move(Move::new(Square::G8, Square::H6, MoveKind::Quiet));
        board2 = board2.do_move(Move::new(Square::E2, Square::E3, MoveKind::Quiet));
        board2 = board2.do_move(Move::new(Square::B8, Square::A6, MoveKind::Quiet));

        assert_eq!(board1, board2);
    }

    #[test]
    fn equal_board_equal_hash() {
        let mut board1 = Board::default();
        board1 = board1.do_move(Move::new(Square::A2, Square::A4, MoveKind::DoublePush));
        board1 = board1.do_move(Move::new(Square::B8, Square::A6, MoveKind::Quiet));
        board1 = board1.do_move(Move::new(Square::E2, Square::E3, MoveKind::Quiet));
        board1 = board1.do_move(Move::new(Square::G8, Square::H6, MoveKind::Quiet));

        let mut board2 =
            Board::from_str("rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq a3 0 1").unwrap();
        board2 = board2.do_move(Move::new(Square::G8, Square::H6, MoveKind::Quiet));
        board2 = board2.do_move(Move::new(Square::E2, Square::E3, MoveKind::Quiet));
        board2 = board2.do_move(Move::new(Square::B8, Square::A6, MoveKind::Quiet));

        assert_eq!(board1.hash(), board2.hash());
    }

    #[test]
    fn not_equal_board_should_not_have_equal_hash() {
        let mut board1 = Board::default();

        let mut board2 =
            Board::from_str("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ")
                .unwrap();

        assert_ne!(board1.hash(), board2.hash());
    }
}
