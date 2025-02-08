use bitboard::Bitboard;
use castling_rights::{Castling, CastlingRights};
use color::Color;
use moves::{Move, MoveKind, Movelist};
use piece::{Piece, PieceKind};
use square::{Direction, File, Rank, Square};

pub mod bitboard;
pub mod castling_rights;
pub mod color;
mod hash;
pub mod moves;
pub mod piece;
pub mod square;

#[derive(Debug, Clone, Copy, Default)]
struct State {
    castling_rights: CastlingRights,
    en_passant_square: Option<Square>,
    fifty_move_counter: u32,
    captured_piece: Option<Piece>,
}

impl State {
    pub fn new(
        cr: CastlingRights,
        ep: Option<Square>,
        fifty: u32,
        captured: Option<Piece>,
    ) -> Self {
        Self {
            castling_rights: cr,
            en_passant_square: ep,
            fifty_move_counter: fifty,
            captured_piece: captured,
        }
    }

    pub fn castling_rights(&self) -> CastlingRights {
        self.castling_rights
    }

    pub fn en_passant_square(&self) -> Option<Square> {
        self.en_passant_square
    }

    pub fn fifty_move_counter(&self) -> u32 {
        self.fifty_move_counter
    }

    pub fn captured_piece(&self) -> Option<Piece> {
        self.captured_piece
    }
}

const MAX_PLIES: usize = 512;

#[derive(Clone)]
pub struct Game {
    squares: [Option<Piece>; 64],
    pieces: [Bitboard; 6],
    colors: [Bitboard; 2],
    color_to_move: Color,
    fullmove_number: u32,
    ply: usize,
    state: [State; MAX_PLIES],
    hashes: [u64; MAX_PLIES],
    moves: [Move; MAX_PLIES],
}

impl Default for Game {
    /// Creates a new game with standard starting position.
    fn default() -> Self {
        Self::new(Self::STARTPOS).unwrap()
    }
}

impl Game {
    pub const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    /// Tries to create a new game from a FEN string.
    ///
    /// ```
    /// use chess::game::Game;
    ///
    /// let game = Game::new(Game::STARTPOS).unwrap();
    /// assert_eq!(game.fen(), Game::STARTPOS);
    ///
    /// ```
    pub fn new(fen: &str) -> Option<Self> {
        let mut game = Self {
            squares: [None; 64],
            pieces: [Bitboard::EMPTY; 6],
            colors: [Bitboard::EMPTY; 2],
            color_to_move: Color::White,
            fullmove_number: 1,
            ply: 0,
            state: [State::default(); MAX_PLIES],
            hashes: [0; MAX_PLIES],
            moves: [Move::default(); MAX_PLIES],
        };

        let mut sections = fen.split_ascii_whitespace();

        let mut rank = Rank::iter().rev().peekable();
        let mut file = File::iter().peekable();

        for c in sections.next()?.chars() {
            match c {
                '/' => {
                    if *rank.peek()? == Rank::First || file.peek().is_some() {
                        return None;
                    }
                    rank.next();
                    file = File::iter().peekable();
                }
                '1'..='8' => {
                    let n = c.to_digit(10).unwrap();
                    (0..n).for_each(|_| _ = file.next());
                }
                _ => {
                    let square = Square::new(*rank.peek()?, file.next()?);
                    let piece = c.try_into().ok()?;
                    game.put_piece(piece, square);
                }
            }
        }

        game.color_to_move = match sections.next()? {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return None,
        };

        let mut cr = CastlingRights::new();

        for c in sections.next()?.chars() {
            match c {
                '-' => (),
                'K' => cr.add(Castling::Kingside(Color::White)),
                'Q' => cr.add(Castling::Queenside(Color::White)),
                'k' => cr.add(Castling::Kingside(Color::Black)),
                'q' => cr.add(Castling::Queenside(Color::Black)),
                _ => return None,
            }
        }

        let ep = match sections.next()? {
            "-" => None,
            ep => {
                let file = ep.chars().nth(0)?.try_into().ok()?;
                let rank = ep.chars().nth(1)?.try_into().ok()?;
                Some(Square::new(rank, file))
            }
        };

        let fifty = sections.next().unwrap_or("0").parse::<u32>().ok()?;
        game.fullmove_number = sections.next().unwrap_or("1").parse::<u32>().ok()?;

        game.state[0] = State::new(cr, ep, fifty, None);

        // Calculate hash

        if game.color_to_move() == Color::Black {
            game.hashes[0] ^= hash::BLACK_HASH;
        }

        game.hashes[0] ^= hash::CR_HASH[game.castling_rights().to_u8() as usize];

        if let Some(ep) = game.en_passant_square() {
            game.hashes[0] ^= hash::EP_HASH[ep.file() as usize];
        }

        Some(game)
    }

    pub fn hash(&self) -> u64 {
        self.hashes[self.ply]
    }

    /// Which side to move?
    ///
    /// ```
    /// use chess::game::{color::Color, Game};
    ///
    /// let game = Game::default();
    /// assert_eq!(game.color_to_move(), Color::White);
    ///
    /// ```
    pub fn color_to_move(&self) -> Color {
        self.color_to_move
    }

    pub fn castling_rights(&self) -> CastlingRights {
        self.state[self.ply].castling_rights()
    }

    pub fn en_passant_square(&self) -> Option<Square> {
        self.state[self.ply].en_passant_square()
    }

    pub fn fullmove_number(&self) -> u32 {
        self.fullmove_number
    }

    pub fn fen(&self) -> String {
        let mut result = String::new();

        // Pieces
        for rank in Rank::iter().rev() {
            let mut empty_squares = 0;
            for file in File::iter() {
                let square = Square::new(rank, file);

                if let Some(piece) = self.get_piece(square) {
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

        result.push(' ');
        result.push(match self.color_to_move() {
            Color::White => 'w',
            Color::Black => 'b',
        });

        result.push(' ');
        if self.castling_rights().is_empty() {
            result.push('-');
        } else {
            if self
                .castling_rights()
                .contains(Castling::Kingside(Color::White))
            {
                result.push('K');
            }
            if self
                .castling_rights()
                .contains(Castling::Queenside(Color::White))
            {
                result.push('Q');
            }
            if self
                .castling_rights()
                .contains(Castling::Kingside(Color::Black))
            {
                result.push('k');
            }
            if self
                .castling_rights()
                .contains(Castling::Queenside(Color::Black))
            {
                result.push('q');
            }
        }

        result.push(' ');
        if let Some(ep) = self.en_passant_square() {
            result.push((ep.file() as u8 + b'a' as u8) as char);
            result.push((ep.rank() as u8 + b'1' as u8) as char);
        } else {
            result.push('-');
        }

        result.push_str(
            format!(" {} {}", self.fifty_move_counter(), self.fullmove_number()).as_str(),
        );

        result
    }

    pub fn moves(&mut self) -> Movelist {
        let mut movelist = Movelist::new();

        self.pawn_moves(&mut movelist);

        let b = (self.pieces_by_kind(PieceKind::Bishop) | self.pieces_by_kind(PieceKind::Queen))
            & self.pieces_by_color(self.color_to_move());

        let r = (self.pieces_by_kind(PieceKind::Rook) | self.pieces_by_kind(PieceKind::Queen))
            & self.pieces_by_color(self.color_to_move());

        let n = self.pieces_by_kind(PieceKind::Knight) & self.pieces_by_color(self.color_to_move());

        self.non_pawn_moves(
            b,
            |f| Bitboard::bishop_attacks(f, self.pieces()),
            &mut movelist,
        );

        self.non_pawn_moves(
            r,
            |f| Bitboard::rook_attacks(f, self.pieces()),
            &mut movelist,
        );

        self.non_pawn_moves(n, |f| Bitboard::knight_attacks(f), &mut movelist);

        self.king_moves(&mut movelist);

        let mut result = Movelist::new();

        for mv in movelist {
            if self.is_legal(mv) {
                result.push(mv);
            }
        }

        result
    }

    fn is_legal(&mut self, mv: Move) -> bool {
        let color_moved = self.color_to_move();
        self.push(mv);
        let is_in_check = self.color_in_check(color_moved);
        self.pop();
        !is_in_check
    }

    fn pawn_moves(&self, movelist: &mut Movelist) {
        let pieces =
            self.pieces_by_kind(PieceKind::Pawn) & self.pieces_by_color(self.color_to_move());

        // Some constants depending on current player
        let (up, capture_east, capture_west, home_rank, promotion_rank) = match self.color_to_move()
        {
            Color::White => (
                Direction::N,
                Direction::NE,
                Direction::NW,
                Bitboard::from(Rank::Second),
                Bitboard::from(Rank::Eighth),
            ),
            Color::Black => (
                Direction::S,
                Direction::SE,
                Direction::SW,
                Bitboard::from(Rank::Seventh),
                Bitboard::from(Rank::First),
            ),
        };

        // Quiet, no promotions
        let quiet = pieces.shift(up) & !self.pieces() & !promotion_rank;

        for to in quiet {
            movelist.push(Move::new(to - up, to, MoveKind::Quiet));
        }

        // Quiet, promotions
        let promotions = pieces.shift(up) & !self.pieces() & promotion_rank;

        // Make lines more shorter/readable
        use MoveKind as MK;

        for to in promotions {
            let from = to - up;
            movelist.push(Move::new(from, to, MK::Promotion(PieceKind::Queen)));
            movelist.push(Move::new(from, to, MK::Promotion(PieceKind::Rook)));
            movelist.push(Move::new(from, to, MK::Promotion(PieceKind::Bishop)));
            movelist.push(Move::new(from, to, MK::Promotion(PieceKind::Knight)));
        }

        // Double push
        let double = (pieces & home_rank).shift(up) & !self.pieces();
        let double = double.shift(up) & !self.pieces();

        for to in double {
            movelist.push(Move::new(to - up - up, to, MK::DoublePush));
        }

        for dir in [capture_east, capture_west] {
            // Captures, no promotions
            let caps =
                pieces.shift(dir) & self.pieces_by_color(!self.color_to_move()) & !promotion_rank;

            for to in caps {
                movelist.push(Move::new(to - dir, to, MK::Capture));
            }

            // Captures, promotions
            let caps =
                pieces.shift(dir) & self.pieces_by_color(!self.color_to_move()) & promotion_rank;

            for to in caps {
                let from = to - dir;
                movelist.push(Move::new(from, to, MK::PromotionCapture(PieceKind::Queen)));
                movelist.push(Move::new(from, to, MK::PromotionCapture(PieceKind::Rook)));
                movelist.push(Move::new(from, to, MK::PromotionCapture(PieceKind::Bishop)));
                movelist.push(Move::new(from, to, MK::PromotionCapture(PieceKind::Knight)));
            }
        }

        // En passant
        if let Some(en_passant_square) = self.en_passant_square() {
            for dir in [capture_east, capture_west] {
                let ep = pieces.shift(dir) & Bitboard::from(en_passant_square);
                for to in ep {
                    movelist.push(Move::new(to - dir, to, MK::EnPassant));
                }
            }
        }
    }

    fn non_pawn_moves<F>(&self, pieces: Bitboard, att: F, movelist: &mut Movelist)
    where
        F: Fn(Square) -> Bitboard,
    {
        for from in pieces {
            let attacks = att(from);
            let quiet = attacks & !self.pieces();
            let captures = attacks & self.pieces_by_color(!self.color_to_move());

            for to in captures {
                movelist.push(Move::new(from, to, MoveKind::Capture));
            }

            for to in quiet {
                movelist.push(Move::new(from, to, MoveKind::Quiet));
            }
        }
    }

    fn king_moves(&self, movelist: &mut Movelist) {
        let from = (self.pieces_by_kind(PieceKind::King)
            & self.pieces_by_color(self.color_to_move()))
        .pop_lsb()
        .unwrap();

        let attacks = Bitboard::king_attacks(from);
        let quiet = attacks & !self.pieces();
        let captures = attacks & self.pieces_by_color(!self.color_to_move());

        for to in captures {
            movelist.push(Move::new(from, to, MoveKind::Capture));
        }

        for to in quiet {
            movelist.push(Move::new(from, to, MoveKind::Quiet));
        }

        if self
            .castling_rights()
            .contains(Castling::Kingside(self.color_to_move()))
        {
            let king_travel_squares = match self.color_to_move() {
                Color::White => [Square::E1, Square::F1, Square::G1],
                Color::Black => [Square::E8, Square::F8, Square::G8],
            };

            let king_travel_safe = king_travel_squares
                .iter()
                .all(|s| !self.is_square_attacked(!self.color_to_move(), *s));

            let castling_squares_clear = match self.color_to_move() {
                Color::White => {
                    self.get_piece(Square::F1).is_none() && self.get_piece(Square::G1).is_none()
                }
                Color::Black => {
                    self.get_piece(Square::F8).is_none() && self.get_piece(Square::G8).is_none()
                }
            };

            if king_travel_safe && castling_squares_clear {
                movelist.push(Move::new(
                    from,
                    from + Direction::E + Direction::E,
                    MoveKind::CastleKingside,
                ));
            }
        }
        if self
            .castling_rights()
            .contains(Castling::Queenside(self.color_to_move()))
        {
            let king_travel_squares = match self.color_to_move() {
                Color::White => [Square::E1, Square::D1, Square::C1],
                Color::Black => [Square::E8, Square::D8, Square::C8],
            };

            let king_travel_safe = king_travel_squares
                .iter()
                .all(|s| !self.is_square_attacked(!self.color_to_move(), *s));

            let castling_squares_clear = match self.color_to_move() {
                Color::White => {
                    self.get_piece(Square::B1).is_none()
                        && self.get_piece(Square::D1).is_none()
                        && self.get_piece(Square::C1).is_none()
                }
                Color::Black => {
                    self.get_piece(Square::B8).is_none()
                        && self.get_piece(Square::D8).is_none()
                        && self.get_piece(Square::C8).is_none()
                }
            };

            if king_travel_safe && castling_squares_clear {
                movelist.push(Move::new(
                    from,
                    from + Direction::W + Direction::W,
                    MoveKind::CastleQueenside,
                ));
            }
        }
    }

    /// Returns the possible piece on `square`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::game::{Game, square::Square, piece::Piece};
    ///
    /// let game = Game::default();
    /// assert_eq!(game.get_piece(Square::A1), Some(Piece::WhiteRook));
    /// ```
    pub fn get_piece(&self, square: Square) -> Option<Piece> {
        self.squares[square as usize]
    }

    /// Puts a `piece` on `square`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::game::{Game, square::Square, piece::Piece};
    ///
    /// let mut game = Game::default();
    /// game.put_piece(Piece::WhiteBishop, Square::E4);
    /// assert_eq!(game.get_piece(Square::E4), Some(Piece::WhiteBishop));
    /// ```
    pub fn put_piece(&mut self, piece: Piece, square: Square) {
        self.squares[square as usize] = Some(piece);

        let bb = Bitboard::from(square);

        self.pieces[piece.kind() as usize] |= bb;
        self.colors[piece.color() as usize] |= bb;

        // Update hash
        self.hashes[self.ply] ^= hash::PIECES_HASH[square as usize][piece as usize];
    }

    /// Removes and returns the possible piece on `square`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::game::piece::{PieceKind, Piece};
    /// use chess::game::square::Square;
    /// use chess::game::Game;
    ///
    /// let mut game = Game::default();
    /// game.put_piece(Piece::WhiteBishop, Square::E4);
    /// assert_eq!(game.take_piece(Square::E4), Some(Piece::WhiteBishop));
    /// assert_eq!(game.take_piece(Square::E4), None);
    /// ```
    pub fn take_piece(&mut self, square: Square) -> Option<Piece> {
        if let Some(piece) = self.squares[square as usize].take() {
            let bb = Bitboard::from(square);

            self.pieces[piece.kind() as usize] &= !bb;
            self.colors[piece.color() as usize] &= !bb;

            // Update hash
            self.hashes[self.ply] ^= hash::PIECES_HASH[square as usize][piece as usize];

            Some(piece)
        } else {
            None
        }
    }

    /// Moves the piece on `from` to `to`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::game::piece::{PieceKind, Piece};
    /// use chess::game::square::Square;
    /// use chess::game::Game;
    ///
    /// let mut game = Game::default();
    /// game.put_piece(Piece::WhiteBishop, Square::E4);
    /// game.move_piece(Square::E4, Square::C2);
    /// assert_eq!(game.take_piece(Square::E4), None);
    /// assert_eq!(game.take_piece(Square::C2), Some(Piece::WhiteBishop));
    /// ```
    pub fn move_piece(&mut self, from: Square, to: Square) {
        if let Some(piece) = self.squares[from as usize].take() {
            let bb = Bitboard::from(from) | Bitboard::from(to);

            self.pieces[piece.kind() as usize] ^= bb;
            self.colors[piece.color() as usize] ^= bb;

            self.squares[to as usize] = Some(piece);

            // Update hash
            self.hashes[self.ply] ^= hash::PIECES_HASH[from as usize][piece as usize];
            self.hashes[self.ply] ^= hash::PIECES_HASH[to as usize][piece as usize];
        }
    }

    /// Returns a bitboard containing all pieces on the board.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::game::square::{Rank, Square};
    /// use chess::game::piece::Piece;
    /// use chess::game::bitboard::Bitboard;
    /// use chess::game::Game;
    ///
    /// let mut game = Game::default();
    /// let bb = Bitboard::from(Rank::First) | Bitboard::from(Rank::Second) | Bitboard::from(Rank::Seventh) | Bitboard::from(Rank::Eighth);
    /// assert_eq!(game.pieces(), bb);
    /// game.put_piece(Piece::WhiteBishop, Square::E4);
    /// game.put_piece(Piece::BlackRook, Square::A5);
    /// assert_eq!(game.pieces(), bb | Bitboard::from(Square::E4) | Bitboard::from(Square::A5));
    /// ```
    pub fn pieces(&self) -> Bitboard {
        self.colors[0] | self.colors[1]
    }

    /// Returns all black and white pieces of `kind`.
    pub fn pieces_by_kind(&self, kind: PieceKind) -> Bitboard {
        self.pieces[kind as usize]
    }

    pub fn pieces_by_color(&self, color: Color) -> Bitboard {
        self.colors[color as usize]
    }

    /// Makes a move on the board.
    pub fn push(&mut self, mv: Move) {
        let from = mv.from();
        let to = mv.to();

        let moved_piece = self.get_piece(from).unwrap();

        let state = self.state[self.ply];
        let mut new_cr = state.castling_rights();
        let mut new_ep = None;
        let mut new_50 = if moved_piece.kind() == PieceKind::Pawn {
            0
        } else {
            state.fifty_move_counter() + 1
        };
        let mut new_captured = None;

        let mut new_hash = self.hash();

        new_hash ^= hash::CR_HASH[self.castling_rights().to_u8() as usize];

        if let Some(ep) = self.en_passant_square() {
            new_hash ^= hash::EP_HASH[ep.file() as usize];
        }

        match mv.kind() {
            moves::MoveKind::Quiet => {
                self.move_piece(from, to);
            }
            moves::MoveKind::Capture => {
                new_captured = self.take_piece(to);
                new_50 = 0;
                self.move_piece(from, to);
            }
            moves::MoveKind::DoublePush => {
                new_ep = Some(Square::from_u8((from as u8 + to as u8) / 2));
                new_hash ^= hash::EP_HASH[new_ep.unwrap().file() as usize];
                self.move_piece(from, to);
            }
            moves::MoveKind::EnPassant => {
                let capture_square = Square::new(from.rank(), to.file());
                new_captured = self.take_piece(capture_square);
                self.move_piece(from, to);
            }
            moves::MoveKind::CastleKingside => {
                let (rfrom, rto) = (
                    Square::from_u8(from as u8 + 3),
                    Square::from_u8(from as u8 + 1),
                );

                self.move_piece(from, to);
                self.move_piece(rfrom, rto);
            }
            moves::MoveKind::CastleQueenside => {
                let (rfrom, rto) = (
                    Square::from_u8(from as u8 - 4),
                    Square::from_u8(from as u8 - 1),
                );
                self.move_piece(from, to);
                self.move_piece(rfrom, rto);
            }
            moves::MoveKind::Promotion(piece_kind) => {
                _ = self.take_piece(from);
                self.put_piece(Piece::new(piece_kind, self.color_to_move()), to);
            }
            moves::MoveKind::PromotionCapture(piece_kind) => {
                _ = self.take_piece(from);
                new_captured = self.take_piece(to);
                self.put_piece(Piece::new(piece_kind, self.color_to_move()), to);
            }
        }

        match from {
            Square::H1 => new_cr.remove(Castling::Kingside(Color::White)),
            Square::A1 => new_cr.remove(Castling::Queenside(Color::White)),
            Square::E1 => {
                new_cr.remove(Castling::Kingside(Color::White));
                new_cr.remove(Castling::Queenside(Color::White));
            }
            Square::H8 => new_cr.remove(Castling::Kingside(Color::Black)),
            Square::A8 => new_cr.remove(Castling::Queenside(Color::Black)),
            Square::E8 => {
                new_cr.remove(Castling::Kingside(Color::Black));
                new_cr.remove(Castling::Queenside(Color::Black));
            }
            _ => (),
        }
        match to {
            Square::H1 => new_cr.remove(Castling::Kingside(Color::White)),
            Square::A1 => new_cr.remove(Castling::Queenside(Color::White)),
            Square::H8 => new_cr.remove(Castling::Kingside(Color::Black)),
            Square::A8 => new_cr.remove(Castling::Queenside(Color::Black)),
            _ => (),
        }

        new_hash ^= hash::CR_HASH[new_cr.to_u8() as usize];

        self.ply += 1;
        self.moves[self.ply] = mv;
        self.state[self.ply] = State::new(new_cr, new_ep, new_50, new_captured);
        self.hashes[self.ply] = new_hash;

        if self.color_to_move() == Color::Black {
            self.fullmove_number += 1;
        }
        self.color_to_move = !self.color_to_move;
    }

    /// Reverts the last move made.
    ///
    /// ```
    ///
    /// ```
    pub fn pop(&mut self) {
        let state = self.state[self.ply];
        let mv = self.moves[self.ply];
        self.ply -= 1;
        let from = mv.from();
        let to = mv.to();

        match mv.kind() {
            moves::MoveKind::Quiet => self.move_piece(to, from),
            moves::MoveKind::Capture => {
                self.move_piece(to, from);
                self.put_piece(state.captured_piece().unwrap(), to);
            }
            moves::MoveKind::DoublePush => {
                self.move_piece(to, from);
            }
            moves::MoveKind::EnPassant => {
                let capture_square = Square::new(from.rank(), to.file());
                self.move_piece(to, from);
                self.put_piece(state.captured_piece().unwrap(), capture_square);
            }
            moves::MoveKind::CastleKingside => {
                let (rfrom, rto) = (
                    Square::from_u8(from as u8 + 3),
                    Square::from_u8(from as u8 + 1),
                );

                self.move_piece(to, from);
                self.move_piece(rto, rfrom);
            }
            moves::MoveKind::CastleQueenside => {
                let (rfrom, rto) = (
                    Square::from_u8(from as u8 - 4),
                    Square::from_u8(from as u8 - 1),
                );
                self.move_piece(to, from);
                self.move_piece(rto, rfrom);
            }
            moves::MoveKind::Promotion(_) => {
                _ = self.take_piece(to);
                self.put_piece(Piece::new(PieceKind::Pawn, !self.color_to_move()), from);
            }
            moves::MoveKind::PromotionCapture(_) => {
                _ = self.take_piece(to);
                self.put_piece(state.captured_piece().unwrap(), to);
                self.put_piece(Piece::new(PieceKind::Pawn, !self.color_to_move()), from);
            }
        }

        if self.color_to_move() == Color::White {
            self.fullmove_number -= 1;
        }
        self.color_to_move = !self.color_to_move;
    }

    pub fn is_check(&self) -> bool {
        self.color_in_check(self.color_to_move())
    }

    pub fn repetition_count(&self) -> u32 {
        let hash = self.hash();
        self.hashes[0..=self.ply]
            .iter()
            .filter(|h| **h == hash)
            .count() as u32
    }

    pub fn fifty_move_counter(&self) -> u32 {
        self.state[self.ply].fifty_move_counter()
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

        let r = Bitboard::rook_attacks(square, self.pieces()) & (queens | rooks);
        let b = Bitboard::bishop_attacks(square, self.pieces()) & (queens | bishops);
        let n = Bitboard::knight_attacks(square) & knights;
        let k = Bitboard::king_attacks(square) & king;
        let p = Bitboard::pawn_attacks(square, !attacker) & pawns;

        (r | b | n | k | p).is_non_empty()
    }
}

pub fn perft(game: &mut Game, depth: usize) -> u64 {
    let moves = game.moves();
    // Bulk count
    if depth == 1 {
        return moves.len() as u64;
    } else if depth == 0 {
        return 1;
    }

    let mut count = 0;
    for mv in moves {
        game.push(mv);
        count += perft(game, depth - 1);
        game.pop();
    }
    count
}

pub fn perft_divide(game: &mut Game, depth: usize) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut count = 0;
    for mv in game.moves() {
        game.push(mv);
        let move_count = perft(game, depth - 1);
        game.pop();
        println!("{}: {}", mv, move_count);
        count += move_count;
    }
    count
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "   | a | b | c | d | e | f | g | h |")?;
        writeln!(f, "   +---+---+---+---+---+---+---+---+---")?;

        for rank in Rank::iter().rev() {
            write!(f, " {} |", rank as usize + 1)?;
            for file in File::iter() {
                let c = if let Some(piece) = self.get_piece(Square::new(rank, file)) {
                    piece.into()
                } else {
                    ' '
                };
                write!(f, " {} |", c)?;
            }
            writeln!(f, " {}", rank as usize + 1)?;
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
    use std::collections::HashMap;

    use super::{
        moves::{Move, MoveKind},
        Game,
    };
    use crate::game::{perft, piece::Piece, square::Square};

    #[test]
    fn transpositions_have_same_hash() {
        let mut game = Game::default();
        game.push(Move::new(Square::A2, Square::A4, MoveKind::DoublePush));
        game.push(Move::new(Square::A7, Square::A5, MoveKind::DoublePush));
        game.push(Move::new(Square::B1, Square::C3, MoveKind::Quiet));
        game.push(Move::new(Square::G8, Square::H6, MoveKind::Quiet));
        let hash1 = game.hash();

        let mut game = Game::default();
        game.push(Move::new(Square::B1, Square::C3, MoveKind::Quiet));
        game.push(Move::new(Square::A7, Square::A5, MoveKind::DoublePush));
        game.push(Move::new(Square::A2, Square::A4, MoveKind::DoublePush));
        game.push(Move::new(Square::G8, Square::H6, MoveKind::Quiet));
        let hash2 = game.hash();

        assert_eq!(hash1, hash2);
    }

    fn positions(game: &mut Game, depth: usize, pos: &mut HashMap<String, u64>) {
        let moves = game.moves();
        // Bulk count
        if depth == 0 {
            let fen = game.fen();
            let fen = fen.split_ascii_whitespace().take(4).collect::<String>();
            if let Some(h) = pos.get(&fen) {
                assert_eq!(*h, game.hash());
            } else {
                pos.insert(fen, game.hash());
            }
            return;
        }

        for mv in moves {
            game.push(mv);

            positions(game, depth - 1, pos);
            game.pop();
        }
    }

    #[test]
    fn transpositions_have_same_hash2() {
        let mut game = Game::default();

        positions(&mut game, 4, &mut HashMap::new());
    }

    #[test]
    fn fen_parsing() {
        let pos =
            Game::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ").unwrap();
        assert_eq!(pos.get_piece(Square::A1), Some(Piece::WhiteRook));
        assert_eq!(pos.get_piece(Square::H1), Some(Piece::WhiteRook));
        assert_eq!(pos.get_piece(Square::B4), Some(Piece::BlackPawn));
        assert_eq!(pos.get_piece(Square::B6), Some(Piece::BlackKnight));
        assert_eq!(pos.get_piece(Square::G7), Some(Piece::BlackBishop));
        assert_eq!(pos.get_piece(Square::E6), Some(Piece::BlackPawn));

        let pos = Game::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -").unwrap();
        assert_eq!(pos.get_piece(Square::E2), Some(Piece::WhitePawn));
        assert_eq!(pos.get_piece(Square::G2), Some(Piece::WhitePawn));
        assert_eq!(pos.get_piece(Square::B4), Some(Piece::WhiteRook));
        assert_eq!(pos.get_piece(Square::F4), Some(Piece::BlackPawn));
        assert_eq!(pos.get_piece(Square::H4), Some(Piece::BlackKing));
        assert_eq!(pos.get_piece(Square::A5), Some(Piece::WhiteKing));
        assert_eq!(pos.get_piece(Square::B5), Some(Piece::WhitePawn));
        assert_eq!(pos.get_piece(Square::H5), Some(Piece::BlackRook));
    }

    fn test_position(line: &str) {
        let mut line = line.split(',');
        let fen = line.next().unwrap().trim();
        let mut game = Game::new(fen).unwrap();
        for (depth, n) in line.enumerate() {
            let depth = depth + 1;
            let actual_count = n.trim().parse::<u64>().unwrap();
            let perft_count = perft(&mut game, depth);
            assert_eq!(perft_count, actual_count, "FEN: {}, depth: {}", fen, depth);
        }
    }

    // Test positions from http://www.rocechess.ch/perft.html

    #[test]
    #[ignore]
    fn perft_test1() {
        include_str!("../perftsuite.txt")
            .lines()
            .take(30)
            .for_each(test_position);
    }

    #[test]
    #[ignore]
    fn perft_test2() {
        include_str!("../perftsuite.txt")
            .lines()
            .skip(30)
            .take(30)
            .for_each(test_position);
    }

    #[test]
    #[ignore]
    fn perft_test3() {
        include_str!("../perftsuite.txt")
            .lines()
            .skip(60)
            .take(30)
            .for_each(test_position);
    }

    #[test]
    #[ignore]
    fn perft_test4() {
        include_str!("../perftsuite.txt")
            .lines()
            .skip(90)
            .for_each(test_position);
    }
}
