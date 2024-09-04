use core::fmt;

use crate::{
    bitboard::Bitboard,
    castling_rights::CastlingRights,
    color::Color,
    moves::{generate_moves, Move, MoveKind},
    piece::{Piece, PieceKind},
    square::{File, Rank, Square},
};

#[derive(Debug)]
pub struct ParseFenError {
    msg: String,
}

impl ParseFenError {
    pub fn new<S: AsRef<str>>(msg: S) -> Self {
        Self {
            msg: msg.as_ref().to_owned(),
        }
    }
}

impl fmt::Display for ParseFenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing FEN: {}.", self.msg)
    }
}

#[derive(Clone)]
pub struct Board {
    bb: [Bitboard; 4],
    state: u32,
}

impl Board {
    /// Creates an empty chessboard.
    pub fn new() -> Self {
        let mut board = Self {
            bb: [Bitboard::EMPTY; 4],
            state: 0,
        };

        board.set_color_to_move(Color::White);
        board.set_en_passant_square(None);
        board.set_halfmove_clock(0);
        board.set_fullmove_number(1);

        board
    }

    /// Creates a new chessboard with the standard starting position.
    pub fn default() -> Self {
        let mut board = Board::new();

        for i in 0..8 {
            board.put(Piece::WhitePawn, Square::from_index(i + 8));
            board.put(Piece::BlackPawn, Square::from_index(i + 48));
        }

        board.put(Piece::WhiteRook, Square::new(Rank::First, File::A));
        board.put(Piece::WhiteKnight, Square::new(Rank::First, File::B));
        board.put(Piece::WhiteBishop, Square::new(Rank::First, File::C));
        board.put(Piece::WhiteQueen, Square::new(Rank::First, File::D));
        board.put(Piece::WhiteKing, Square::new(Rank::First, File::E));
        board.put(Piece::WhiteBishop, Square::new(Rank::First, File::F));
        board.put(Piece::WhiteKnight, Square::new(Rank::First, File::G));
        board.put(Piece::WhiteRook, Square::new(Rank::First, File::H));

        board.put(Piece::BlackRook, Square::new(Rank::Eighth, File::A));
        board.put(Piece::BlackKnight, Square::new(Rank::Eighth, File::B));
        board.put(Piece::BlackBishop, Square::new(Rank::Eighth, File::C));
        board.put(Piece::BlackQueen, Square::new(Rank::Eighth, File::D));
        board.put(Piece::BlackKing, Square::new(Rank::Eighth, File::E));
        board.put(Piece::BlackBishop, Square::new(Rank::Eighth, File::F));
        board.put(Piece::BlackKnight, Square::new(Rank::Eighth, File::G));
        board.put(Piece::BlackRook, Square::new(Rank::Eighth, File::H));

        // Full castling rights
        board.add_castling_rights(CastlingRights::Kingside(Color::White));
        board.add_castling_rights(CastlingRights::Queenside(Color::White));
        board.add_castling_rights(CastlingRights::Kingside(Color::Black));
        board.add_castling_rights(CastlingRights::Queenside(Color::Black));

        board
    }

    /// Creates a new chessboard from FEN string.
    /// # Example
    /// ```
    /// # use chess::{board::Board};
    /// let board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    /// assert_eq!(board.fen(), "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    /// let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
    /// assert_eq!(board.fen(), "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    /// ```
    pub fn from_fen(fen: &str) -> Result<Board, ParseFenError> {
        let mut board = Board::new();

        let mut fen = fen.split_whitespace();

        fn parse_piece(c: char) -> Option<Piece> {
            match c {
                'P' => Some(Piece::WhitePawn),
                'N' => Some(Piece::WhiteKnight),
                'B' => Some(Piece::WhiteBishop),
                'R' => Some(Piece::WhiteRook),
                'Q' => Some(Piece::WhiteQueen),
                'K' => Some(Piece::WhiteKing),
                'p' => Some(Piece::BlackPawn),
                'n' => Some(Piece::BlackKnight),
                'b' => Some(Piece::BlackBishop),
                'r' => Some(Piece::BlackRook),
                'q' => Some(Piece::BlackQueen),
                'k' => Some(Piece::BlackKing),
                _ => None,
            }
        }

        // pieces
        let pieces = fen
            .next()
            .ok_or(ParseFenError::new("No pieces specified"))?;
        let mut rank = Rank::iter().rev().peekable();
        let mut file = File::iter().peekable();

        for c in pieces.chars() {
            match c {
                '1'..='8' => {
                    let n = c as u8 - '0' as u8;
                    for _ in 0..n {
                        file.next();
                    }
                }
                '/' => {
                    rank.next();
                    file = File::iter().peekable();
                }
                c => match parse_piece(c) {
                    Some(piece) => {
                        let &r = rank
                            .peek()
                            .ok_or(ParseFenError::new("Invalid rank in pieces"))?;
                        let &f = file
                            .peek()
                            .ok_or(ParseFenError::new("Invalid file in pieces"))?;
                        let square = Square::new(r, f);
                        board.put(piece, square);
                        file.next();
                    }
                    None => {
                        return Err(ParseFenError::new(format!("Unexpected character '{}'", c)))
                    }
                },
            }
        }

        // color
        let color = fen.next().ok_or(ParseFenError::new("No color specified"))?;
        let color = match color {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(ParseFenError::new(format!("Unexpected color '{}'", color))),
        };
        board.set_color_to_move(color);

        // castling rights
        for c in fen
            .next()
            .ok_or(ParseFenError::new("No castling rights specified"))?
            .chars()
        {
            match c {
                'K' => board.add_castling_rights(CastlingRights::Kingside(Color::White)),
                'Q' => board.add_castling_rights(CastlingRights::Queenside(Color::White)),
                'k' => board.add_castling_rights(CastlingRights::Kingside(Color::Black)),
                'q' => board.add_castling_rights(CastlingRights::Queenside(Color::Black)),
                '-' => (),
                _ => return Err(ParseFenError::new(format!("Unexpected character '{}'", c))),
            }
        }

        // en passant square
        let ep = fen
            .next()
            .ok_or(ParseFenError::new("No en passant square specified"))?;

        board.set_en_passant_square(if ep == "-" {
            None
        } else {
            Some(
                ep.parse()
                    .map_err(|_| ParseFenError::new("Invalid en passant square"))?,
            )
        });

        if let Some(s) = fen.next() {
            board.set_halfmove_clock(s.parse::<usize>().unwrap());
        }

        if let Some(s) = fen.next() {
            board.set_fullmove_number(s.parse::<usize>().unwrap());
        }

        Ok(board)
    }

    /// Returns the position in FEN.
    /// # Example
    /// ```
    /// # use chess::{board::Board};
    /// let board = Board::default();
    /// assert_eq!(board.fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    pub fn fen(&self) -> String {
        let mut result = String::new();

        // pieces
        for rank in Rank::iter().rev() {
            let mut empty_squares = 0;
            for file in File::iter() {
                let square = Square::new(rank, file);

                match self.at(square) {
                    Some(piece) => {
                        if empty_squares > 0 {
                            result.push(('0' as u8 + empty_squares) as char);
                            empty_squares = 0;
                        }
                        let c = match piece {
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
                        };
                        result.push(c);
                    }
                    None => empty_squares += 1,
                }
            }
            if empty_squares > 0 {
                result.push(('0' as u8 + empty_squares) as char);
            }
            if rank != Rank::First {
                result.push('/');
            }
        }

        // color
        result.push(' ');
        result.push(match self.color_to_move() {
            Color::White => 'w',
            Color::Black => 'b',
        });

        // castling rights
        result.push(' ');

        if self.no_castlings() {
            result.push('-');
        }

        if self.has_castling_rights(CastlingRights::Kingside(Color::White)) {
            result.push('K');
        }
        if self.has_castling_rights(CastlingRights::Queenside(Color::White)) {
            result.push('Q');
        }
        if self.has_castling_rights(CastlingRights::Kingside(Color::Black)) {
            result.push('k');
        }
        if self.has_castling_rights(CastlingRights::Queenside(Color::Black)) {
            result.push('q');
        }

        // en passant square
        result.push(' ');

        match self.en_passant_square() {
            None => result.push('-'),
            Some(square) => {
                result.push(('a' as u8 + square.file().to_index() as u8) as char);
                result.push(('1' as u8 + square.rank().to_index() as u8) as char);
            }
        }

        // halfmove clock + fullmove number
        result.push_str(format!(" {} {}", self.halfmove_clock(), self.fullmove_number()).as_str());

        result
    }

    /// Returns the possible piece on `square`.
    /// # Example
    /// ```
    /// # use chess::{board::Board, square::Square, piece::Piece};
    /// assert_eq!(Board::default().at(Square::E4), None);
    /// assert_eq!(Board::default().at(Square::A1), Some(Piece::WhiteRook));
    /// assert_eq!(Board::default().at(Square::B7), Some(Piece::BlackPawn));
    /// ```
    #[inline(always)]
    pub fn at(&self, square: Square) -> Option<Piece> {
        // https://www.chessprogramming.org/Quad-Bitboards
        let code = ((self.bb[0].to_u64() >> square.to_index()) & 1)
            + 2 * ((self.bb[1].to_u64() >> square.to_index()) & 1)
            + 4 * ((self.bb[2].to_u64() >> square.to_index()) & 1)
            + 8 * ((self.bb[3].to_u64() >> square.to_index()) & 1);

        const CODES: [Option<Piece>; 14] = [
            None,                     // 0
            None,                     // 1
            Some(Piece::WhitePawn),   // 2
            Some(Piece::BlackPawn),   // 3
            Some(Piece::WhiteKnight), // 4
            Some(Piece::BlackKnight), // 5
            Some(Piece::WhiteBishop), // 6
            Some(Piece::BlackBishop), // 7
            Some(Piece::WhiteRook),   // 8
            Some(Piece::BlackRook),   // 9
            Some(Piece::WhiteQueen),  // 10
            Some(Piece::BlackQueen),  // 11
            Some(Piece::WhiteKing),   // 12
            Some(Piece::BlackKing),   // 13
        ];

        CODES[code as usize]
    }

    /// Puts `piece` on `square`.
    /// # Example
    /// ```
    /// # use chess::{board::Board, square::Square, piece::Piece};
    /// let mut board = Board::new();
    /// assert_eq!(board.at(Square::F5), None);
    /// board.put(Piece::WhiteKnight, Square::F5);
    /// assert_eq!(board.at(Square::F5), Some(Piece::WhiteKnight));
    ///
    /// ```
    #[inline(always)]
    pub fn put(&mut self, piece: Piece, square: Square) {
        let bb = Bitboard::new(square);
        match piece.color() {
            Color::Black => self.bb[0] |= bb,
            _ => (),
        }

        match piece.kind() {
            PieceKind::Pawn => self.bb[1] |= bb,
            PieceKind::Knight => self.bb[2] |= bb,
            PieceKind::Bishop => {
                self.bb[1] |= bb;
                self.bb[2] |= bb;
            }
            PieceKind::Rook => self.bb[3] |= bb,
            PieceKind::Queen => {
                self.bb[1] |= bb;
                self.bb[3] |= bb;
            }
            PieceKind::King => {
                self.bb[2] |= bb;
                self.bb[3] |= bb;
            }
        }
    }

    /// Returns the player with the next move.
    #[inline(always)]
    pub fn color_to_move(&self) -> Color {
        match self.state & 1 {
            0 => Color::White,
            _ => Color::Black,
        }
    }

    /// The current move number
    #[inline(always)]
    pub fn fullmove_number(&self) -> usize {
        (self.state >> 16) as usize
    }

    /// Number of half-turns since pawn move or capture.
    #[inline(always)]
    pub fn halfmove_clock(&self) -> usize {
        ((self.state >> 9) & 127) as usize
    }

    #[inline(always)]
    pub(crate) fn has_castling_rights(&self, cr: CastlingRights) -> bool {
        let bits = (self.state as u8 >> 1) & 15;
        bits & cr.bitmask() != 0
    }

    pub fn can_castle_kingside(&self, color: Color) -> bool {
        self.has_castling_rights(CastlingRights::Kingside(color))
    }

    pub fn can_castle_queenside(&self, color: Color) -> bool {
        self.has_castling_rights(CastlingRights::Queenside(color))
    }

    fn no_castlings(&self) -> bool {
        (self.state as u8 >> 1) & 15 == 0
    }

    /// Returns en passant square, if last move was a pawn double push.
    #[inline(always)]
    pub fn en_passant_square(&self) -> Option<Square> {
        let ep_file = (self.state as usize >> 5) & 15;
        if ep_file == 8 {
            None
        } else {
            // determine ep rank with previous player
            let rank = match !self.color_to_move() {
                Color::White => Rank::Third,
                Color::Black => Rank::Sixth,
            };

            Some(Square::new(rank, File::from_index(ep_file)))
        }
    }

    #[inline(always)]
    pub(crate) fn pieces(&self) -> Bitboard {
        self.bb[1] | self.bb[2] | self.bb[3]
    }

    #[inline(always)]
    pub(crate) fn pieces_by_color(&self, color: Color) -> Bitboard {
        match color {
            Color::Black => self.bb[0],
            Color::White => self.bb[0] ^ self.pieces(),
        }
    }

    #[inline(always)]
    fn odd_pieces(&self) -> Bitboard {
        self.bb[1] ^ self.bb[2] ^ self.bb[3]
    }

    #[inline(always)]
    pub(crate) fn pieces_by_kind(&self, kind: PieceKind) -> Bitboard {
        match kind {
            PieceKind::Pawn => self.bb[1] & self.odd_pieces(),
            PieceKind::Knight => self.bb[2] & self.odd_pieces(),
            PieceKind::Bishop => self.bb[1] & self.bb[2],
            PieceKind::Rook => self.bb[3] & self.odd_pieces(),
            PieceKind::Queen => self.bb[1] & self.bb[3],
            PieceKind::King => self.bb[2] & self.bb[3],
        }
    }

    /// Sets the player with the next move.
    #[inline(always)]
    pub fn set_color_to_move(&mut self, color: Color) {
        match color {
            Color::Black => self.state |= 1,
            Color::White => self.state &= !1,
        }
    }

    /// Sets the current move number
    #[inline(always)]
    pub fn set_fullmove_number(&mut self, fullmove: usize) {
        self.state = self.state & !(0xffff << 16) | ((fullmove as u32) << 16);
    }

    #[inline(always)]
    pub fn increment_fullmove_number(&mut self) {
        self.set_fullmove_number(self.fullmove_number() + 1);
    }

    /// Sets the number of half-turns since pawn move or capture.
    #[inline(always)]
    pub fn set_halfmove_clock(&mut self, halfmove: usize) {
        self.state = self.state & !(127 << 9) | ((halfmove as u32) << 9);
    }

    #[inline(always)]
    pub(crate) fn add_castling_rights(&mut self, cr: CastlingRights) {
        self.state |= (cr.bitmask() as u32) << 1;
    }

    // #[inline(always)]
    // pub(crate) fn remove_castling_rights(&mut self, cr: CastlingRights) {
    //     self.state &= !((cr.bitmask() as u32) << 1);
    // }

    pub fn add_castling_kingside(&mut self, color: Color) {
        self.add_castling_rights(CastlingRights::Kingside(color));
    }

    pub fn add_castling_queenside(&mut self, color: Color) {
        self.add_castling_rights(CastlingRights::Queenside(color));
    }

    #[inline(always)]
    pub fn set_en_passant_square(&mut self, ep: Option<Square>) {
        let bits = match ep {
            Some(square) => square.file().to_index() as u32,
            None => 8,
        };
        self.state = self.state & !(15 << 5) | (bits << 5);
    }

    /// Executes a move and updates the board state.
    /// Returns an undo object which is used for unmaking the move.
    pub fn do_move(&self, mv: Move) -> Board {
        let mut board = self.clone();

        let from = mv.from();
        let to = mv.to();

        board.set_en_passant_square(None);
        let halfmove = match board.at(from) {
            Some(piece) if piece.kind() == PieceKind::Pawn => 0,
            _ => board.halfmove_clock() + 1,
        };
        board.set_halfmove_clock(halfmove);

        match mv.kind() {
            MoveKind::Quiet => {
                board.move_piece(from, to);
            }
            MoveKind::Cap => {
                board.take_piece(to);
                board.move_piece(from, to);
                board.set_halfmove_clock(0);
            }
            MoveKind::Double => {
                let ep = Some(Square::from_index((from.to_index() + to.to_index()) / 2));
                board.set_en_passant_square(ep);
                board.move_piece(from, to);
            }
            MoveKind::EnPassant => {
                let capsq = Square::new(from.rank(), to.file());
                board.take_piece(capsq);
                board.move_piece(from, to);
            }
            MoveKind::Castling => {
                let (rook_from, rook_to) = if to.to_index() > from.to_index() {
                    (
                        Square::from_index(from.to_index() + 3),
                        Square::from_index(from.to_index() + 1),
                    )
                } else {
                    (
                        Square::from_index(from.to_index() - 4),
                        Square::from_index(from.to_index() - 1),
                    )
                };

                board.move_piece(from, to);
                board.move_piece(rook_from, rook_to);
            }
            MoveKind::PromQueen => {
                board.take_piece(from);
                board.put(Piece::new(PieceKind::Queen, board.color_to_move()), to);
            }
            MoveKind::PromRook => {
                board.take_piece(from);
                board.put(Piece::new(PieceKind::Rook, board.color_to_move()), to);
            }
            MoveKind::PromBishop => {
                board.take_piece(from);
                board.put(Piece::new(PieceKind::Bishop, board.color_to_move()), to);
            }
            MoveKind::PromKnight => {
                board.take_piece(from);
                board.put(Piece::new(PieceKind::Knight, board.color_to_move()), to);
            }

            MoveKind::PromCapQueen => {
                board.take_piece(to);

                board.take_piece(from);
                board.put(Piece::new(PieceKind::Queen, board.color_to_move()), to);
            }
            MoveKind::PromCapRook => {
                board.take_piece(to);

                board.take_piece(from);
                board.put(Piece::new(PieceKind::Rook, board.color_to_move()), to);
            }
            MoveKind::PromCapBishop => {
                board.take_piece(to);

                board.take_piece(from);
                board.put(Piece::new(PieceKind::Bishop, board.color_to_move()), to);
            }
            MoveKind::PromCapKnight => {
                board.take_piece(to);

                board.take_piece(from);
                board.put(Piece::new(PieceKind::Knight, board.color_to_move()), to);
            }
        }

        // Remove castling rights
        let mut new_cr = (board.state as u8 >> 1) & 15;

        new_cr &= !Self::cr_affected(from);
        new_cr &= !Self::cr_affected(to);

        board.state = board.state & !(15 << 1) | ((new_cr as u32) << 1);

        match board.color_to_move() {
            Color::Black => board.increment_fullmove_number(),
            _ => (),
        }

        board.set_color_to_move(!board.color_to_move());

        board
    }

    fn cr_affected(square: Square) -> u8 {
        match square.to_index() {
            0 => 2,
            4 => 3,
            7 => 1,
            56 => 8,
            60 => 12,
            63 => 4,
            _ => 0,
        }
    }

    pub fn is_in_check(&self) -> bool {
        let king_square = (self.pieces_by_kind(PieceKind::King)
            & self.pieces_by_color(self.color_to_move()))
        .first()
        .unwrap();

        let enemy = self.pieces_by_color(!self.color_to_move());

        let pieces = self.pieces_by_kind(PieceKind::Pawn) & enemy;
        if (Bitboard::pawn_attacks(pieces, !self.color_to_move()) & Bitboard::new(king_square))
            .is_non_empty()
        {
            return true;
        }

        let pieces = self.pieces_by_kind(PieceKind::Knight) & enemy;
        if (Bitboard::knight_attacks(king_square) & pieces).is_non_empty() {
            return true;
        }

        let pieces = (self.pieces_by_kind(PieceKind::Bishop)
            | self.pieces_by_kind(PieceKind::Queen))
            & enemy;
        if (Bitboard::bishop_attacks(king_square, self.pieces()) & pieces).is_non_empty() {
            return true;
        }

        let pieces =
            (self.pieces_by_kind(PieceKind::Rook) | self.pieces_by_kind(PieceKind::Queen)) & enemy;
        if (Bitboard::rook_attacks(king_square, self.pieces()) & pieces).is_non_empty() {
            return true;
        }

        false
    }

    /// Removes and returns the piece on `square`.
    fn take_piece(&mut self, square: Square) -> Option<Piece> {
        let piece = self.at(square).unwrap();

        let bb = Bitboard::new(square);
        match piece.color() {
            Color::Black => self.bb[0] ^= bb,
            _ => (),
        }

        match piece.kind() {
            PieceKind::Pawn => self.bb[1] ^= bb,
            PieceKind::Knight => self.bb[2] ^= bb,
            PieceKind::Bishop => {
                self.bb[1] ^= bb;
                self.bb[2] ^= bb;
            }
            PieceKind::Rook => self.bb[3] ^= bb,
            PieceKind::Queen => {
                self.bb[1] ^= bb;
                self.bb[3] ^= bb;
            }
            PieceKind::King => {
                self.bb[2] ^= bb;
                self.bb[3] ^= bb;
            }
        }

        Some(piece)
    }

    /// Moves the piece on `from` to `to`. Destination square is assumed to be empty!
    fn move_piece(&mut self, from: Square, to: Square) {
        let piece = self.at(from).unwrap();

        let bb = Bitboard::new(from) | to;
        match piece.color() {
            Color::Black => self.bb[0] ^= bb,
            _ => (),
        }

        match piece.kind() {
            PieceKind::Pawn => self.bb[1] ^= bb,
            PieceKind::Knight => self.bb[2] ^= bb,
            PieceKind::Bishop => {
                self.bb[1] ^= bb;
                self.bb[2] ^= bb;
            }
            PieceKind::Rook => self.bb[3] ^= bb,
            PieceKind::Queen => {
                self.bb[1] ^= bb;
                self.bb[3] ^= bb;
            }
            PieceKind::King => {
                self.bb[2] ^= bb;
                self.bb[3] ^= bb;
            }
        }
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        generate_moves(self).into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn castling_rights() {
        let board = Board::default();
        assert!(board.has_castling_rights(CastlingRights::Kingside(Color::White)));
        assert!(board.has_castling_rights(CastlingRights::Queenside(Color::White)));
        assert!(board.has_castling_rights(CastlingRights::Kingside(Color::Black)));
    }

    #[test]
    fn board_take_piece() {
        let mut board = Board::default();
        assert_eq!(
            board.at(Square::new(Rank::Second, File::A)),
            Some(Piece::WhitePawn)
        );
        let p = board.take_piece(Square::new(Rank::Second, File::A));
        assert_eq!(board.at(Square::new(Rank::Second, File::A)), None);
        assert_eq!(p, Some(Piece::WhitePawn));
    }

    #[test]
    fn board_move_piece() {
        let mut board = Board::default();
        assert_eq!(
            board.at(Square::new(Rank::Seventh, File::B)),
            Some(Piece::BlackPawn)
        );
        board.move_piece(
            Square::new(Rank::Seventh, File::B),
            Square::new(Rank::Sixth, File::B),
        );
        assert_eq!(board.at(Square::new(Rank::Seventh, File::B)), None);
        assert_eq!(
            board.at(Square::new(Rank::Sixth, File::B)),
            Some(Piece::BlackPawn)
        );
    }
}
