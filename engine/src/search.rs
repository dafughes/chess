use chess::board::Board;

use crate::{eval, value::Value};

fn evaluate(board: &Board) -> Value {
    eval::material(board)
}

pub fn negamax(board: Board, depth: usize, start_depth: usize) -> Value {
    let moves = board.moves();

    if moves.is_empty() {
        if board.in_check() {
            // Return a Mate with negative value since the current player is losing.
            return -Value::mate(start_depth - depth);
        } else {
            return Value::Draw;
        }
    } else if depth == 0 {
        return evaluate(&board);
    }

    moves
        .into_iter()
        .map(|mv| -negamax(board.do_move(mv), depth - 1, start_depth))
        .max()
        .unwrap_or_default()
}
