use std::{fmt, ops, str::FromStr};

use crate::{
    bitboard::{Bitboard, Direction},
    board::Board,
    castling_rights::CastlingRights,
    color::Color,
    piece::PieceKind,
    square::{File, Rank, Square},
};

#[derive(Debug, Clone, Copy)]
pub enum MoveKind {
    Quiet,
    Cap,
    Double,
    EnPassant,
    Castling,
    PromKnight,
    PromBishop,
    PromRook,
    PromQueen,
    PromCapKnight,
    PromCapBishop,
    PromCapRook,
    PromCapQueen,
}

impl MoveKind {
    /*pub(crate) fn is_promotion(self) -> bool {
        match self {
            MoveKind::Quiet
            | MoveKind::Cap
            | MoveKind::Double
            | MoveKind::EnPassant
            | MoveKind::Castling => false,
            _ => true,
        }
    }*/

    pub(crate) fn is_capture(self) -> bool {
        match self {
            MoveKind::Cap
            | MoveKind::EnPassant
            | MoveKind::PromCapQueen
            | MoveKind::PromCapRook
            | MoveKind::PromCapBishop
            | MoveKind::PromCapKnight => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Move(u16);

// #[derive(Debug, Clone, Copy)]
// pub struct Move {
//     from: Square,
//     to: Square,
//     kind: MoveKind,
// }

impl Move {
    pub fn new(from: Square, to: Square, kind: MoveKind) -> Self {
        let bits = from.to_index() | (to.to_index() << 6) | Self::movekind_to_bits(kind);
        Self(bits as u16)
    }

    #[inline(always)]
    pub fn from(&self) -> Square {
        Square::from_index(self.0 as usize & 63)
    }

    #[inline(always)]
    pub fn to(&self) -> Square {
        Square::from_index((self.0 as usize >> 6) & 63)
    }

    #[inline(always)]
    pub fn kind(&self) -> MoveKind {
        Self::bits_to_movekind(self.0 & 0xF000)
    }

    pub fn promotion_kind(&self) -> Option<PieceKind> {
        match self.kind() {
            MoveKind::PromCapQueen | MoveKind::PromQueen => Some(PieceKind::Queen),
            MoveKind::PromCapRook | MoveKind::PromRook => Some(PieceKind::Rook),
            MoveKind::PromCapBishop | MoveKind::PromBishop => Some(PieceKind::Bishop),
            MoveKind::PromCapKnight | MoveKind::PromKnight => Some(PieceKind::Knight),
            _ => None,
        }
    }

    fn bits_to_movekind(bits: u16) -> MoveKind {
        match bits >> 12 {
            0 => MoveKind::Quiet,
            1 => MoveKind::Cap,
            2 => MoveKind::Double,
            3 => MoveKind::EnPassant,
            4 => MoveKind::Castling,
            5 => MoveKind::PromKnight,
            6 => MoveKind::PromBishop,
            7 => MoveKind::PromRook,
            8 => MoveKind::PromQueen,
            9 => MoveKind::PromCapKnight,
            10 => MoveKind::PromCapBishop,
            11 => MoveKind::PromCapRook,
            12 => MoveKind::PromCapQueen,
            _ => unreachable!(),
        }
    }

    fn movekind_to_bits(kind: MoveKind) -> usize {
        (kind as usize) << 12
    }

    #[inline(always)]
    pub fn null() -> Self {
        Self(0)
    }

    pub fn to_index(&self) -> u16 {
        self.0
    }

    pub fn from_index(i: u16) -> Self {
        Self(i)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            char::from(self.from().file()),
            char::from(self.from().rank()),
            char::from(self.to().file()),
            char::from(self.to().rank())
        )?;
        match self.promotion_kind() {
            Some(piece) => write!(f, "{}", char::from(piece)),
            None => Ok(()),
        }
    }
}

const MAX_MOVES_FROM_POSITION: usize = 256;
pub struct Movelist {
    moves: [Move; MAX_MOVES_FROM_POSITION],
    count: usize,
}

impl ops::Index<usize> for Movelist {
    type Output = Move;
    fn index(&self, index: usize) -> &Self::Output {
        &self.moves[index]
    }
}

impl Movelist {
    pub fn new() -> Self {
        Self {
            moves: [Move::null(); MAX_MOVES_FROM_POSITION],
            count: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn push(&mut self, mv: Move) {
        self.moves[self.count] = mv;
        self.count += 1;
    }
}

impl<'a> IntoIterator for &'a Movelist {
    type Item = Move;
    type IntoIter = MovelistIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MovelistIntoIter {
            list: &self,
            index: 0,
        }
    }
}

pub struct MovelistIntoIter<'a> {
    list: &'a Movelist,
    index: usize,
}

impl<'a> Iterator for MovelistIntoIter<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.list.count {
            None
        } else {
            let index = self.index;
            self.index += 1;
            Some(self.list.moves[index])
        }
    }
}

fn pawn_moves(board: &Board, pieces: Bitboard, allowed_squares: Bitboard, moves: &mut Movelist) {
    let (up, home_rank, promotion_rank) = match board.color_to_move() {
        Color::White => (
            Direction::N,
            Bitboard::rank(Rank::Second),
            Bitboard::rank(Rank::Eighth),
        ),
        Color::Black => (
            Direction::S,
            Bitboard::rank(Rank::Seventh),
            Bitboard::rank(Rank::First),
        ),
    };

    let all = board.pieces();
    let enemy = board.pieces_by_color(!board.color_to_move());
    // quiet
    let dest = pieces.shift(up) & !all & allowed_squares;

    // promotions
    for to in dest & promotion_rank {
        let from = to - up;
        moves.push(Move::new(from, to, MoveKind::PromQueen));
        moves.push(Move::new(from, to, MoveKind::PromRook));
        moves.push(Move::new(from, to, MoveKind::PromBishop));
        moves.push(Move::new(from, to, MoveKind::PromKnight));
    }

    // non-promotions
    for to in dest & !promotion_rank {
        let from = to - up;
        moves.push(Move::new(from, to, MoveKind::Quiet));
    }

    // double
    let dest = (pieces & home_rank).shift(up) & !all;
    let dest = dest.shift(up) & !all & allowed_squares;
    for to in dest {
        let from = to - up - up;
        moves.push(Move::new(from, to, MoveKind::Double));
    }

    // captures
    let capture_directions = match board.color_to_move() {
        Color::White => [Direction::NE, Direction::NW],
        Color::Black => [Direction::SE, Direction::SW],
    };

    for dir in capture_directions {
        let dest = pieces.shift(dir) & enemy & allowed_squares;
        // promotions
        for to in dest & promotion_rank {
            let from = to - dir;
            moves.push(Move::new(from, to, MoveKind::PromCapQueen));
            moves.push(Move::new(from, to, MoveKind::PromCapRook));
            moves.push(Move::new(from, to, MoveKind::PromCapBishop));
            moves.push(Move::new(from, to, MoveKind::PromCapKnight));
        }

        // non-promotions
        for to in dest & !promotion_rank {
            let from = to - dir;
            moves.push(Move::new(from, to, MoveKind::Cap));
        }
    }

    // en passant
    let ep_bb = board
        .en_passant_square()
        .map(|s| Bitboard::new(s))
        .unwrap_or(Bitboard::EMPTY);

    for dir in capture_directions {
        let dest = pieces.shift(dir) & ep_bb & allowed_squares;
        for to in dest {
            let from = to - dir;
            moves.push(Move::new(from, to, MoveKind::EnPassant));
        }
    }
}

fn add_moves<F>(
    board: &Board,
    pieces: Bitboard,
    allowed_squares: Bitboard,
    attacks: F,
    moves: &mut Movelist,
) where
    F: Fn(Square) -> Bitboard,
{
    let enemy = board.pieces_by_color(!board.color_to_move());

    for from in pieces {
        let dest = attacks(from) & allowed_squares;
        for to in dest & !enemy {
            moves.push(Move::new(from, to, MoveKind::Quiet));
        }
        for to in dest & enemy {
            moves.push(Move::new(from, to, MoveKind::Cap));
        }
    }
}

/// Returns:
/// - Attacked squares
/// - Attacked squares with defending king masked out, i.e. squares where king can move
/// - Pieces giving check
fn calculate_attacks(board: &Board, attacker: Color) -> (Bitboard, Bitboard, Bitboard) {
    let mut attacks = Bitboard::EMPTY;
    let mut attacks_through_king = Bitboard::EMPTY;
    let mut checking_pieces = Bitboard::EMPTY;

    let defender = !attacker;

    let attacking_pieces = board.pieces_by_color(attacker);
    let defending_pieces = board.pieces_by_color(!attacker);
    let all = board.pieces();

    let defender_king_bb = board.pieces_by_kind(PieceKind::King) & defending_pieces;

    // pawns
    let pieces = board.pieces_by_kind(PieceKind::Pawn) & attacking_pieces;
    attacks |= Bitboard::pawn_attacks(pieces, attacker);
    if (attacks & defender_king_bb).is_non_empty() {
        checking_pieces |= Bitboard::pawn_attacks(defender_king_bb, defender) & pieces;
    }

    // knights
    let pieces = board.pieces_by_kind(PieceKind::Knight) & attacking_pieces;
    for from in pieces {
        let att = Bitboard::knight_attacks(from);

        if (att & defender_king_bb).is_non_empty() {
            checking_pieces |= from;
        }

        attacks |= att;
    }

    // bishop/queens
    let pieces = (board.pieces_by_kind(PieceKind::Bishop) | board.pieces_by_kind(PieceKind::Queen))
        & attacking_pieces;

    for from in pieces {
        let att = Bitboard::bishop_attacks(from, all);

        if (att & defender_king_bb).is_non_empty() {
            checking_pieces |= from;
        }

        // Attacks through king
        attacks_through_king |= Bitboard::bishop_attacks(from, all ^ defender_king_bb);
        attacks |= att;
    }

    // rook/queens
    let pieces = (board.pieces_by_kind(PieceKind::Rook) | board.pieces_by_kind(PieceKind::Queen))
        & attacking_pieces;
    for from in pieces {
        let att = Bitboard::rook_attacks(from, all);

        if (att & defender_king_bb).is_non_empty() {
            checking_pieces |= from;
        }

        // Attacks through king
        attacks_through_king |= Bitboard::rook_attacks(from, all ^ defender_king_bb);
        attacks |= att;
    }

    // king
    let pieces = board.pieces_by_kind(PieceKind::King) & attacking_pieces;
    for from in pieces {
        attacks |= Bitboard::king_attacks(from);
    }

    attacks_through_king |= attacks;

    (attacks, attacks_through_king, checking_pieces)
}

fn pinned_moves(b: &Board, p: Bitboard, a: Bitboard, moves: &mut Movelist) {
    let all: Bitboard = b.pieces();

    let square = p.first().unwrap();
    match b.at(square).unwrap().kind() {
        PieceKind::Pawn => pawn_moves(b, p, a, moves),
        PieceKind::Bishop => add_moves(b, p, a, |from| Bitboard::bishop_attacks(from, all), moves),
        PieceKind::Rook => add_moves(b, p, a, |from| Bitboard::rook_attacks(from, all), moves),
        PieceKind::Queen => add_moves(
            b,
            p,
            a,
            |from| Bitboard::bishop_attacks(from, all) | Bitboard::rook_attacks(from, all),
            moves,
        ),
        _ => (), // Knights can't move if pinned and king being pinned doesn't even make sense
    }
}

pub fn generate_moves(board: &Board) -> Movelist {
    let mut moves = Movelist::new();

    let us = board.color_to_move();
    let them = !us;

    let friendly = board.pieces_by_color(us);
    let enemy = board.pieces_by_color(them);
    let all: Bitboard = board.pieces();

    // let pawns = board.pieces_by_kind(PieceKind::Pawn);
    let knights = board.pieces_by_kind(PieceKind::Knight);
    let bishops = board.pieces_by_kind(PieceKind::Bishop);
    let rooks = board.pieces_by_kind(PieceKind::Rook);
    let queens = board.pieces_by_kind(PieceKind::Queen);

    let king_square = (board.pieces_by_kind(PieceKind::King) & friendly)
        .first()
        .unwrap();

    let (attacks, attacks_through_king, checking_pieces) = calculate_attacks(board, them);

    let num_checkers = checking_pieces.popcount();

    if num_checkers == 2 {
        // Only king evasions
        add_moves(
            board,
            friendly & king_square,
            !attacks_through_king & !friendly,
            |from| Bitboard::king_attacks(from),
            &mut moves,
        );
        return moves;
    }

    let mut allowed_squares = Bitboard::EMPTY;

    // Slightly complicated for pawns...
    let mut pawn_allowed_squares = Bitboard::EMPTY;

    if num_checkers == 1 {
        // If one checker, pieces other than king can only block checks or capture checking pieces
        let mut blocking_squares = Bitboard::EMPTY;
        for from in checking_pieces {
            blocking_squares |= Bitboard::between(from, king_square);
        }
        allowed_squares |= !friendly & (blocking_squares | checking_pieces);
        pawn_allowed_squares |= allowed_squares;

        // En passantable checker
        match board.en_passant_square() {
            Some(ep) => {
                let pawn = Bitboard::new(ep).shift(them.up());
                if (pawn & checking_pieces).is_non_empty() {
                    pawn_allowed_squares |= ep;
                }
            }
            None => (),
        }
    } else {
        // Else pieces can move anywhere except capture own pieces
        allowed_squares = !friendly;
        pawn_allowed_squares = allowed_squares;

        match board.en_passant_square() {
            Some(ep) => {
                pawn_allowed_squares |= ep;
            }
            None => (),
        }
    };

    // Check pinned pieces
    let mut pinned_pieces = Bitboard::EMPTY;

    // All potential pinners
    let pinners = (((bishops | queens) & Bitboard::bishop_mask(king_square))
        | ((rooks | queens) & Bitboard::rook_mask(king_square)))
        & enemy;

    // Handle possible en passant scenarios differently
    if let Some(ep_square) = board.en_passant_square() {
        for square in pinners {
            let between = Bitboard::between(king_square, square);
            let pinned = between & all;

            let num_pinned = pinned.popcount();

            // If there is only one our piece between the slider and our king, the piece is pinned and we process its moves now.
            if num_pinned == 1 && (pinned & friendly).is_non_empty() {
                // Remove piece from normal move generation
                pinned_pieces |= pinned;

                // Piece can only move between the pinner and king, or capture it.
                let allowed = between | square;

                pinned_moves(board, pinned, allowed_squares & allowed, &mut moves);
            }

            // Possible en passantable enemy pawn
            let pawn = Bitboard::new(ep_square + them.up());

            // En passant pin
            // Opponent pawn that is between our king and an enemy slider
            // And if ep square is not between king and slider
            if pinned == pawn && (between & ep_square).is_empty() {
                // We can't en passant
                pawn_allowed_squares &= !Bitboard::new(ep_square);
            }

            // Case where 2 pawns, ours and enemys are between our king and an enemy slider -> can't en passant
            if num_pinned == 2 && (pinned & pawn).is_non_empty() && (between & ep_square).is_empty()
            {
                pawn_allowed_squares &= !Bitboard::new(ep_square);
            }
        }
    } else {
        for square in pinners {
            let between = Bitboard::between(king_square, square);
            let pinned = between & all;

            let num_pinned = pinned.popcount();

            // If there is only one our piece between the slider and our king, the piece is pinned and we process its moves now.
            if num_pinned == 1 && (pinned & friendly).is_non_empty() {
                // Remove piece from normal move generation
                pinned_pieces |= pinned;

                // Piece can only move between the pinner and king, or capture it.
                let allowed = between | square;

                pinned_moves(board, pinned, allowed_squares & allowed, &mut moves);
            }
        }
    }

    // Moves for remaining pieces
    let pieces = board.pieces_by_kind(PieceKind::Pawn) & friendly & !pinned_pieces;
    pawn_moves(board, pieces, pawn_allowed_squares, &mut moves);

    let pieces = knights & friendly & !pinned_pieces;
    add_moves(
        board,
        pieces,
        allowed_squares,
        |from| Bitboard::knight_attacks(from),
        &mut moves,
    );

    let pieces = (bishops | queens) & friendly & !pinned_pieces;
    add_moves(
        board,
        pieces,
        allowed_squares,
        |from| Bitboard::bishop_attacks(from, all),
        &mut moves,
    );

    let pieces = (rooks | queens) & friendly & !pinned_pieces;
    add_moves(
        board,
        pieces,
        allowed_squares,
        |from| Bitboard::rook_attacks(from, all),
        &mut moves,
    );

    let pieces = friendly & king_square;
    add_moves(
        board,
        pieces,
        !attacks_through_king & !friendly,
        |from| Bitboard::king_attacks(from),
        &mut moves,
    );

    // Castlings
    let (king_travel, middle_squares) = match us {
        Color::White => (Bitboard::from_u64(28), Bitboard::from_u64(14)),
        Color::Black => (Bitboard::from_u64(28 << 56), Bitboard::from_u64(14 << 56)),
    };

    let can_castle = board.has_castling_rights(CastlingRights::Queenside(us))
        && ((king_travel & attacks) | (middle_squares & all)).is_empty();

    if can_castle {
        moves.push(Move::new(
            king_square,
            Square::from_index(king_square.to_index() - 2),
            MoveKind::Castling,
        ));
    }

    let (king_travel, middle_squares) = match us {
        Color::White => (Bitboard::from_u64(112), Bitboard::from_u64(96)),
        Color::Black => (Bitboard::from_u64(112 << 56), Bitboard::from_u64(96 << 56)),
    };

    let can_castle = board.has_castling_rights(CastlingRights::Kingside(us))
        && ((king_travel & attacks) | (middle_squares & all)).is_empty();

    if can_castle {
        moves.push(Move::new(
            king_square,
            Square::from_index(king_square.to_index() + 2),
            MoveKind::Castling,
        ));
    }

    moves
}
