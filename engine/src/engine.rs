use std::{
    io::BufRead,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::JoinHandle,
    time::Instant,
};

use chess::{
    board::{moves::perft_divide, Board},
    uci::{parse_move, Command, SearchParams},
};

pub mod random;

pub trait ChessEngine {
    fn name() -> String;

    fn author() -> String;

    fn search(board: Board, history: Vec<Board>, params: SearchParams, stop: Arc<AtomicBool>);
}

pub fn main_loop<E: ChessEngine>() {
    let stdin = std::io::stdin();

    let stop = Arc::new(AtomicBool::new(false));
    let mut search_handle: Option<JoinHandle<()>> = None;

    let mut board = Board::default();
    let mut history: Vec<Board> = vec![];

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        match line.parse::<Command>() {
            Ok(command) => match command {
                Command::Uci => {
                    println!("uciok");
                    println!("id name {}", E::name());
                    println!("id author {}", E::author());
                }
                Command::UciNewGame => (),
                Command::IsReady => println!("readyok"),
                Command::Position(fen, moves) => {
                    history.clear();
                    if let Ok(mut b) = fen.parse::<Board>() {
                        history.push(b.clone());
                        for m in moves {
                            if let Ok(mv) = parse_move(&b, &m) {
                                b = b.do_move(mv);
                                history.push(b.clone());
                            } else {
                                b = Board::default();
                                history.clear();
                                history.push(b.clone());
                                break;
                            }
                        }

                        board = b;
                    } else {
                        board = Board::default();
                        history.push(board.clone());
                    }
                }
                Command::Go(params) => {
                    stop.store(false, Ordering::Relaxed);

                    search_handle = Some(std::thread::spawn({
                        let stop_clone = stop.clone();
                        let board_clone = board.clone();
                        let history_clone = history.clone();
                        || E::search(board_clone, history_clone, params, stop_clone)
                    }));
                }
                Command::Stop => stop.store(true, Ordering::Relaxed),
                Command::Quit => {
                    stop.store(true, Ordering::Relaxed);
                    if let Some(handle) = search_handle.take() {
                        _ = handle.join();
                    }
                    break;
                }
                Command::Display => println!("{}", board),
                Command::Perft(depth) => {
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
