use crate::{board::Board, color::Color, piece::PieceKind};

pub type Score = i32;

pub const INF: Score = 1000000;
pub const MATE: Score = 100000;
pub const DRAW: Score = 0;

const P: [(PieceKind, i32); 5] = [
    (PieceKind::Pawn, 100),
    (PieceKind::Knight, 300),
    (PieceKind::Bishop, 300),
    (PieceKind::Rook, 500),
    (PieceKind::Queen, 900),
];



pub fn evaluate(board: &Board) -> Score {
    // Count material value

    let white = board.pieces_by_color(Color::White);
    let black = board.pieces_by_color(Color::Black);

    let mut score = 0;

    for &(kind, value) in P.iter() {
        let pieces = board.pieces_by_kind(kind);
        let diff = (white & pieces).popcount() as i32 - (black & pieces).popcount() as i32;

        score += diff * value;
    }

    // Evaluate from the perspective of the player who just moved
    let color_moved = board.color_to_move();
    match color_moved {
        Color::White => score,
        Color::Black => -score,
    }
}
