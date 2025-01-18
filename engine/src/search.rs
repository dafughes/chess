use chess::board::Board;

use crate::{eval, value::Value};

fn evaluate(board: &Board) -> Value {
    eval::material(board)
}

pub fn negamax(board: Board, history: &mut Vec<Board>, depth: usize, start_depth: usize) -> Value {
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

    // 50-move rule
    if board.halfmove_clock() >= 100 {
        return Value::Draw;
    }

    let mut best: Option<Value> = None;

    for mv in moves {
        let new_board = board.do_move(mv);

        // Fifty-move rule
        // If history of board states contains `new_board` two times -> draw
        if history.iter().filter(|b| **b == new_board).count() == 2 {
            return Value::Draw;
        }

        history.push(new_board.clone());
        let value = -negamax(new_board, history, depth - 1, start_depth);

        best = Some(best.unwrap_or(value).max(value));

        history.pop();
    }

    best.unwrap_or_default()
}
