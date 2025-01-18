use chess::board::{color::Color, piece::PieceKind, Board};

use crate::value::Value;

/// Evaluates the position from current players point of view.
/// For example, evaluation of -1000 with Black being the color to move means that White is winning.
pub fn material(board: &Board) -> Value {
    // Material value
    let white = board.pieces_by_color(Color::White);
    let black = board.pieces_by_color(Color::Black);

    let mut value = 0;

    // TODO: exclude king
    for kind in PieceKind::iter() {
        let kind_value = match kind {
            PieceKind::Pawn => 100,
            PieceKind::Knight => 300,
            PieceKind::Bishop => 300,
            PieceKind::Rook => 500,
            PieceKind::Queen => 900,
            PieceKind::King => 0,
        };
        let pieces = board.pieces_by_kind(kind);
        let balance = (pieces & white).popcount() as i32 - (pieces & black).popcount() as i32;

        value += balance * kind_value;
    }

    if board.color_to_move() == Color::Black {
        value = -value;
    }

    Value::Evaluation(value)
}
