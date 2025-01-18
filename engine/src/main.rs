use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use chess::board::{color::Color, moves::Move, Board};
use engine::Engine;
use search::negamax;
use value::{MoveWithValue, Value};

pub mod engine;
pub mod eval;
pub mod search;
pub mod uci;
pub mod value;

pub struct IterativeDeepening1;

impl Engine for IterativeDeepening1 {
    fn name() -> String {
        String::from("Iterative deepening 1")
    }

    fn author() -> String {
        String::from("Sam")
    }

    fn search(board: Board, moves: Vec<Move>, params: uci::SearchParams, stop: Arc<AtomicBool>) {
        // This engine starts searching from depth 1 and deepens the search while search time left.

        // Calculate target search time
        let average_plies_per_game = 80;
        let plies_to_go = average_plies_per_game - moves.len();

        // Search parameters should contain remaining time
        let time_remaining = match board.color_to_move() {
            Color::White => params.wtime.unwrap_or(60000),
            Color::Black => params.btime.unwrap_or(60000),
        };

        let search_time = time_remaining as f64 / plies_to_go as f64;

        let search_time = Duration::from_millis(search_time as u64);
        let start_time = Instant::now();

        let mut outer_best_move: Option<MoveWithValue> = None;

        let moves = board.moves();
        if moves.is_empty() {
            println!("bestmove {}", outer_best_move.unwrap_or_default().mv);
            return;
        }

        for depth in 1.. {
            let mut inner_best_move: Option<MoveWithValue> = None;

            for mv in &moves {
                // Check time
                if (Instant::now() - start_time) >= search_time {
                    println!("bestmove {}", outer_best_move.unwrap_or_default().mv);
                    return;
                }
                // Check if commanded to stop
                if stop.load(Ordering::Relaxed) {
                    println!("bestmove {}", outer_best_move.unwrap_or_default().mv);
                    return;
                }

                let value = -negamax(board.do_move(mv), depth - 1, depth);

                let mvw = MoveWithValue { mv, value };
                if let Some(best) = inner_best_move {
                    if mvw > best {
                        println!("info depth {} score {}", depth, mvw.value);
                        inner_best_move = Some(mvw);
                    }
                } else {
                    println!("info depth {} score {}", depth, mvw.value);
                    inner_best_move = Some(mvw);
                }
                // If mate is found, return immediately
                match value {
                    Value::Mate(n) if n >= 0 => {
                        println!("bestmove {}", mv);
                        return;
                    }
                    _ => (),
                }
            }

            outer_best_move = inner_best_move;
        }

        println!("bestmove {}", outer_best_move.unwrap_or_default().mv)
    }
}

fn main() {
    engine::main_loop::<IterativeDeepening1>();
}
