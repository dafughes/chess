use crate::{
    board::Board,
    eval::{evaluate, Score, DRAW, INF, MATE},
    moves::{generate_moves, Move},
    uci::SearchParams,
};

pub fn negamax(board: &Board, depth: usize, depth_left: usize) -> Score {
    let moves = generate_moves(board);

    if depth_left == 0 {
        if moves.is_empty() {
            if board.is_in_check() {
                return -MATE + (depth as i32);
            } else {
                return DRAW;
            }
        } else {
            return evaluate(board);
        }
    } else {
        if moves.is_empty() {
            if board.is_in_check() {
                return -MATE + (depth as i32);
            } else {
                return DRAW;
            }
        }
    }

    let mut max = -INF;

    for mv in &moves {
        let score = -negamax(&board.do_move(mv), depth + 1, depth_left - 1);
        if score > max {
            max = score;
        }
    }

    max
}

pub fn negamax_alphabeta(
    board: &Board,
    mut alpha: Score,
    beta: Score,
    depth: usize,
    depth_left: usize,
) -> Score {
    let moves = generate_moves(board);

    if depth_left == 0 {
        if moves.is_empty() {
            if board.is_in_check() {
                return -MATE + (depth as i32);
            } else {
                return DRAW;
            }
        } else {
            // return evaluate(board);
            return quiescence_search(board, alpha, beta);
        }
    } else {
        if moves.is_empty() {
            if board.is_in_check() {
                return -MATE + (depth as i32);
            } else {
                return DRAW;
            }
        }
    }

    for mv in &moves {
        let score =
            -negamax_alphabeta(&board.do_move(mv), -beta, -alpha, depth + 1, depth_left - 1);
        if score >= beta {
            return beta;
        }

        if score > alpha {
            alpha = score;
        }
    }

    return alpha;
}

pub fn quiescence_search(board: &Board, mut alpha: Score, beta: Score) -> Score {
    let standing_pat = evaluate(board);

    if standing_pat >= beta {
        return beta;
    }
    if alpha < standing_pat {
        alpha = standing_pat;
    }

    let moves = generate_moves(board);

    for mv in &moves {
        if mv.kind().is_capture() {
            let score = -quiescence_search(&board.do_move(mv), -beta, -alpha);

            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }
    }

    return alpha;
}
