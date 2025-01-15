use crate::bitboard::Bitboard;

use super::{
    castlingrights::Castling,
    color::Color,
    piece::PieceKind,
    square::{Direction, Rank, Square},
    Board,
};

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind() {
            MoveKind::Promotion(kind) | MoveKind::PromotionCapture(kind) => {
                write!(f, "{}{}{}", self.from(), self.to(), char::from(kind))
            }
            _ => write!(f, "{}{}", self.from(), self.to()),
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

    pub(crate) fn push(&mut self, mv: Move) {
        self.moves[self.len] = mv;
        self.len += 1;
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

impl FromIterator<Move> for Movelist {
    fn from_iter<T: IntoIterator<Item = Move>>(iter: T) -> Self {
        let mut movelist = Self::new();
        for mv in iter {
            movelist.moves[movelist.len] = mv;
            movelist.len += 1;
        }
        movelist
    }
}

impl Board {
    pub fn moves(&self) -> Movelist {
        let mut movelist = Movelist::new();

        self.pawn_moves(&mut movelist);
        self.knight_moves(&mut movelist);
        self.bishop_moves(&mut movelist);
        self.rook_moves(&mut movelist);
        self.king_moves(&mut movelist);

        movelist
            .into_iter()
            .filter(|mv| self.is_legal(*mv))
            .collect()
    }

    fn is_legal(&self, mv: Move) -> bool {
        let color_moved = self.color_to_move();
        let board = self.do_move(mv);
        !board.color_in_check(color_moved)
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

    fn knight_moves(&self, movelist: &mut Movelist) {
        let pieces =
            self.pieces_by_kind(PieceKind::Knight) & self.pieces_by_color(self.color_to_move());

        for from in pieces {
            let attacks = Bitboard::knight_attacks(from);
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

    fn bishop_moves(&self, movelist: &mut Movelist) {
        let pieces = (self.pieces_by_kind(PieceKind::Bishop)
            | self.pieces_by_kind(PieceKind::Queen))
            & self.pieces_by_color(self.color_to_move());

        for from in pieces {
            let attacks = Bitboard::bishop_attacks(from, self.pieces());
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

    fn rook_moves(&self, movelist: &mut Movelist) {
        let pieces = (self.pieces_by_kind(PieceKind::Rook) | self.pieces_by_kind(PieceKind::Queen))
            & self.pieces_by_color(self.color_to_move());

        for from in pieces {
            let attacks = Bitboard::rook_attacks(from, self.pieces());
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

        // TODO: castlings
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
                Color::White => self.get(Square::F1).is_none() && self.get(Square::G1).is_none(),
                Color::Black => self.get(Square::F8).is_none() && self.get(Square::G8).is_none(),
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
                    self.get(Square::B1).is_none()
                        && self.get(Square::D1).is_none()
                        && self.get(Square::C1).is_none()
                }
                Color::Black => {
                    self.get(Square::B8).is_none()
                        && self.get(Square::D8).is_none()
                        && self.get(Square::C8).is_none()
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
}

pub fn perft(board: &Board, depth: usize) -> u64 {
    let moves = board.moves();
    // Bulk count
    if depth == 1 {
        return moves.len() as u64;
    } else if depth == 0 {
        return 1;
    }

    let mut count = 0;
    for mv in moves {
        count += perft(&board.do_move(mv), depth - 1);
    }
    count
}

pub fn perft_divide(board: &Board, depth: usize) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut count = 0;
    for mv in board.moves() {
        let move_count = perft(&board.do_move(mv), depth - 1);
        println!("{}: {}", mv, move_count);
        count += move_count;
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::board::moves::perft;

    fn test_position(line: &str) {
        let mut line = line.split(',');
        let fen = line.next().unwrap().trim();
        let board = fen.parse().unwrap();
        for (depth, n) in line.enumerate() {
            let depth = depth + 1;
            let actual_count = n.trim().parse::<u64>().unwrap();
            let perft_count = perft(&board, depth);
            assert_eq!(perft_count, actual_count, "FEN: {}, depth: {}", fen, depth);
        }
    }

    // Test positions from http://www.rocechess.ch/perft.html
    #[test]
    #[ignore]
    fn perft_test() {
        include_str!("../../perftsuite.txt")
            .lines()
            .for_each(test_position);
    }
}
