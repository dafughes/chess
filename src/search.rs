use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    board::Board,
    eval::{evaluate, Score, DRAW, INF, MATE},
    moves::{generate_moves, Move},
};

#[wasm_bindgen]
#[derive(Debug)]
pub struct SearchParams {
    pub wtime: u32,
    pub btime: u32,
    pub winc: u32,
    pub binc: u32,
    pub movestogo: u32,
    pub depth: u32,
    pub nodes: u32,
    pub mate: u32,
    pub movetime: u32,
    pub infinite: bool,
}

#[wasm_bindgen]
impl SearchParams {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            wtime: 0,
            btime: 0,
            winc: 0,
            binc: 0,
            movestogo: 0,
            depth: 0,
            nodes: 0,
            mate: 0,
            movetime: 0,
            infinite: false,
        }
    }
}

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

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn search(params: SearchParams, board: &Board) -> Move {
    let moves = generate_moves(board);

    if params.depth == 0 {
        return Move::null();
    }

    let mut max = -INF;
    let mut bestmove = Move::null();

    for mv in &moves {
        let score = -negamax_alphabeta(&board.do_move(mv), -INF, INF, 1, params.depth as usize - 1);

        log(&format!("{}: {}", mv, score));
        // println!("{}: {}", mv, score);
        if score > max {
            max = score;
            bestmove = mv;
        }
    }
    log("");

    bestmove
}
