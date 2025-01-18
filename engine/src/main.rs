use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use chess::{
    board::{color::Color, Board},
    uci::SearchParams,
};
use engine::ChessEngine;
use rand::Rng;
use search::negamax;
use value::{MoveWithValue, Value};

pub mod engine;
pub mod eval;
pub mod search;
pub mod value;

pub struct Engine;

impl ChessEngine for Engine {
    fn name() -> String {
        String::from("Engine v2")
    }

    fn author() -> String {
        String::from("Sam")
    }

    fn search(board: Board, mut history: Vec<Board>, params: SearchParams, stop: Arc<AtomicBool>) {
        // This engine starts searching from depth 1 and deepens the search while search time left.

        // Calculate target search time
        let average_moves_per_game = 75;
        let moves_to_go = average_moves_per_game - history.len() / 2;

        // Search parameters should contain remaining time
        let time_remaining = match board.color_to_move() {
            Color::White => params.wtime.unwrap_or(60000),
            Color::Black => params.btime.unwrap_or(60000),
        };

        let search_time = time_remaining as f64 / moves_to_go as f64;

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

            let mut nodes = 0;

            for mv in &moves {
                // println!("depth {}, move {}", depth, mv);
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

                let new_board = board.do_move(mv);

                history.push(new_board.clone());
                let value = -negamax(new_board, &mut history, depth - 1, depth, &mut nodes);
                history.pop();

                let mvw = MoveWithValue { mv, value };
                if let Some(best) = inner_best_move {
                    if mvw > best {
                        let elapsed = (Instant::now() - start_time).as_secs_f64();
                        let nps = nodes as f64 / elapsed;
                        println!(
                            "info depth {} score {} nps {}",
                            depth, mvw.value, nps as u64
                        );
                        inner_best_move = Some(mvw);
                    } else if mvw.value == best.value {
                        // Sprinkle some randomness to a chess engine's dull life
                        if rand::thread_rng().gen_bool(0.5) {
                            inner_best_move = Some(mvw);
                            let elapsed = (Instant::now() - start_time).as_secs_f64();
                            let nps = nodes as f64 / elapsed;
                            println!(
                                "info depth {} score {} nps {}",
                                depth, mvw.value, nps as u64
                            );
                        }
                    }
                } else {
                    let elapsed = (Instant::now() - start_time).as_secs_f64();
                    let nps = nodes as f64 / elapsed;
                    println!(
                        "info depth {} score {} nps {}",
                        depth, mvw.value, nps as u64
                    );
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

        println!("bestmove {}", outer_best_move.unwrap_or_default().mv);
    }
}

fn main() {
    engine::main_loop::<Engine>();
}
