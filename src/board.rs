use crate::{
    bitboard::Bitboard,
    castling_rights::CastlingRights,
    color::Color,
    moves::{Move, MoveKind},
    piece::{Piece, PieceKind},
    square::{File, Rank, Square},
};

#[derive(Clone)]
pub struct Board {
    bb: [Bitboard; 4],
    state: u32,
}

impl Default for Board {
    /// Creates a chessboard with standard starting position.
    fn default() -> Self {
        let mut board = Board::new();

        for i in 0..8 {
            board.put(Piece::WhitePawn, Square::from_index(i + 8));
            board.put(Piece::BlackPawn, Square::from_index(i + 48));
        }

        board.put(Piece::WhiteRook, Square::A1);
        board.put(Piece::WhiteKnight, Square::B1);
        board.put(Piece::WhiteBishop, Square::C1);
        board.put(Piece::WhiteQueen, Square::D1);
        board.put(Piece::WhiteKing, Square::E1);
        board.put(Piece::WhiteBishop, Square::F1);
        board.put(Piece::WhiteKnight, Square::G1);
        board.put(Piece::WhiteRook, Square::H1);

        board.put(Piece::BlackRook, Square::A8);
        board.put(Piece::BlackKnight, Square::B8);
        board.put(Piece::BlackBishop, Square::C8);
        board.put(Piece::BlackQueen, Square::D8);
        board.put(Piece::BlackKing, Square::E8);
        board.put(Piece::BlackBishop, Square::F8);
        board.put(Piece::BlackKnight, Square::G8);
        board.put(Piece::BlackRook, Square::H8);

        // Full castling rights
        board.add_castling_rights(CastlingRights::Kingside(Color::White));
        board.add_castling_rights(CastlingRights::Queenside(Color::White));
        board.add_castling_rights(CastlingRights::Kingside(Color::Black));
        board.add_castling_rights(CastlingRights::Queenside(Color::Black));

        board
    }
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

    /// Creates a new chessboard from FEN string.
    /// # Example
    /// ```
    /// # use chess::{board::Board};
    /// let board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    /// assert_eq!(board.fen(), "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    /// let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    /// assert_eq!(board.fen(), "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    /// ```
    pub fn from_fen(fen: &str) -> Self {
        let mut board = Board::new();

        let mut fen = fen.split_whitespace();

        // TODO: proper error handling for invalid FEN strings.
        // pieces
        let mut square = Square::A8.to_index() as isize;
        for c in fen.next().unwrap().chars() {
            match c {
                '1'..='8' => square += (c as u8 - '1' as u8) as isize,
                'P' => board.put(Piece::WhitePawn, Square::from_index(square as usize)),
                'N' => board.put(Piece::WhiteKnight, Square::from_index(square as usize)),
                'B' => board.put(Piece::WhiteBishop, Square::from_index(square as usize)),
                'R' => board.put(Piece::WhiteRook, Square::from_index(square as usize)),
                'Q' => board.put(Piece::WhiteQueen, Square::from_index(square as usize)),
                'K' => board.put(Piece::WhiteKing, Square::from_index(square as usize)),
                'p' => board.put(Piece::BlackPawn, Square::from_index(square as usize)),
                'n' => board.put(Piece::BlackKnight, Square::from_index(square as usize)),
                'b' => board.put(Piece::BlackBishop, Square::from_index(square as usize)),
                'r' => board.put(Piece::BlackRook, Square::from_index(square as usize)),
                'q' => board.put(Piece::BlackQueen, Square::from_index(square as usize)),
                'k' => board.put(Piece::BlackKing, Square::from_index(square as usize)),
                '/' => square -= 17,
                _ => (),
            }

            square += 1;
        }

        // color
        let color = match fen.next().unwrap() {
            "w" => Color::White,
            "b" => Color::Black,
            _ => Color::White,
        };
        board.set_color_to_move(color);

        // castling rights
        for c in fen.next().unwrap().chars() {
            match c {
                'K' => board.add_castling_rights(CastlingRights::Kingside(Color::White)),
                'Q' => board.add_castling_rights(CastlingRights::Queenside(Color::White)),
                'k' => board.add_castling_rights(CastlingRights::Kingside(Color::Black)),
                'q' => board.add_castling_rights(CastlingRights::Queenside(Color::Black)),
                _ => (),
            }
        }

        // en passant square
        let section = fen.next().unwrap();

        if section != "-" {
            let f = File::from_index((section.chars().nth(0).unwrap() as u8 - 'a' as u8) as usize);
            let r = Rank::from_index((section.chars().nth(1).unwrap() as u8 - '1' as u8) as usize);
            board.set_en_passant_square(Some(Square::new(r, f)));
        }

        if let Some(s) = fen.next() {
            board.set_halfmove_clock(s.parse::<usize>().unwrap());
        }

        if let Some(s) = fen.next() {
            board.set_fullmove_number(s.parse::<usize>().unwrap());
        }

        board
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
        for r in (0..8).rev() {
            let mut empty_squares = 0;
            for f in 0..8 {
                let square = Square::from_index(r * 8 + f);
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
            if r > 0 {
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

    /// ```
    /// # use chess::{board::Board, color::Color, castling_rights::CastlingRights};
    /// let board = Board::default();
    /// assert!(board.has_castling_rights( CastlingRights::Kingside(Color::White)));
    /// assert!(board.has_castling_rights(CastlingRights::Queenside(Color::White)));
    /// assert!(board.has_castling_rights( CastlingRights::Kingside(Color::Black)));
    /// assert!(board.has_castling_rights(CastlingRights::Queenside(Color::Black)));
    /// ```
    #[inline(always)]
    pub fn has_castling_rights(&self, cr: CastlingRights) -> bool {
        let bits = (self.state as u8 >> 1) & 15;
        bits & cr.bitmask() != 0
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
    pub fn pieces(&self) -> Bitboard {
        self.bb[1] | self.bb[2] | self.bb[3]
    }

    #[inline(always)]
    pub fn pieces_by_color(&self, color: Color) -> Bitboard {
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
    pub fn pieces_by_kind(&self, kind: PieceKind) -> Bitboard {
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
    pub fn add_castling_rights(&mut self, cr: CastlingRights) {
        self.state |= (cr.bitmask() as u32) << 1;
    }

    #[inline(always)]
    pub fn remove_castling_rights(&mut self, cr: CastlingRights) {
        self.state &= !((cr.bitmask() as u32) << 1);
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
            MoveKind::Prom(piece) => {
                board.take_piece(from);
                board.put(Piece::new(piece, board.color_to_move()), to);
            }
            MoveKind::PromCap(piece) => {
                board.take_piece(to);

                board.take_piece(from);
                board.put(Piece::new(piece, board.color_to_move()), to);
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
        match square {
            Square::A1 => 2,
            Square::E1 => 3,
            Square::H1 => 1,
            Square::A8 => 8,
            Square::E8 => 12,
            Square::H8 => 4,
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
    #[inline(always)]
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
    #[inline(always)]
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_take_piece() {
        let mut board = Board::default();
        assert_eq!(board.at(Square::A2), Some(Piece::WhitePawn));
        let p = board.take_piece(Square::A2);
        assert_eq!(board.at(Square::A2), None);
        assert_eq!(p, Some(Piece::WhitePawn));
    }

    #[test]
    fn board_move_piece() {
        let mut board = Board::default();
        assert_eq!(board.at(Square::B7), Some(Piece::BlackPawn));
        board.move_piece(Square::B7, Square::B6);
        assert_eq!(board.at(Square::B7), None);
        assert_eq!(board.at(Square::B6), Some(Piece::BlackPawn));
    }
}
