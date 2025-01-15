use std::{
    io::BufRead,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::JoinHandle,
    time::Instant,
};

use chess::board::{
    moves::{perft_divide, Move},
    Board,
};

pub mod random;

use crate::uci::{self, SearchParams};

pub trait Engine {
    fn name() -> String;

    fn author() -> String;

    fn search(board: Board, moves: Vec<Move>, params: SearchParams, stop: Arc<AtomicBool>);
}

pub fn main_loop<E: Engine>() {
    let stdin = std::io::stdin();

    let stop = Arc::new(AtomicBool::new(false));
    let mut search_handle: Option<JoinHandle<()>> = None;

    let mut board = Board::default();
    let mut played_moves: Vec<Move> = vec![];

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        match line.parse::<uci::Command>() {
            Ok(command) => match command {
                uci::Command::Uci => {
                    println!("uciok");
                    println!("id name {}", E::name());
                    println!("id author {}", E::author());
                }
                uci::Command::IsReady => println!("readyok"),
                uci::Command::Position(fen, moves) => {
                    if let Ok(mut b) = fen.parse() {
                        for m in moves {
                            if let Ok(mv) = uci::parse_move(&b, &m) {
                                b = b.do_move(mv);
                                played_moves.push(mv);
                            } else {
                                b = Board::default();
                                played_moves.clear();
                                break;
                            }
                        }

                        board = b;
                    } else {
                        board = Board::default();
                    }
                }
                uci::Command::Go(params) => {
                    stop.store(false, Ordering::Relaxed);

                    search_handle = Some(std::thread::spawn({
                        let stop_clone = stop.clone();
                        let board_clone = board.clone();
                        let moves_clone = played_moves.clone();
                        || E::search(board_clone, moves_clone, params, stop_clone)
                    }));
                }
                uci::Command::Stop => stop.store(true, Ordering::Relaxed),
                uci::Command::Quit => {
                    stop.store(true, Ordering::Relaxed);
                    if let Some(handle) = search_handle.take() {
                        _ = handle.join();
                    }
                    break;
                }
                uci::Command::Display => println!("{}", board),
                uci::Command::Perft(depth) => {
                    let t0 = Instant::now();
                    let nodes = perft_divide(&board, depth);
                    let elapsed = (Instant::now() - t0).as_secs_f64();
                    let nps = nodes as f64 / elapsed;
                    let (nps, prefix) = if nps > 1e6 {
                        (nps / 1e6, "M")
                    } else if nps > 1e3 {
                        (nps / 1e3, "k")
                    } else {
                        (nps, "")
                    };
                    println!(
                        "Leaf nodes searched: {}, time elapsed: {:.2} s, {:.2} {}nps",
                        nodes, elapsed, nps, prefix
                    );
                }
                _ => (),
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}
