use std::{
    io::BufRead,
    sync::{atomic::AtomicBool, Arc},
    time::{Duration, Instant},
};

use chess::{
    board::Board,
    color::Color,
    debug::perft_divide,
    eval::INF,
    moves::{generate_moves, Move},
    search::negamax_alphabeta,
    uci::{self, SearchParams, UCICommand},
};

#[derive(Debug, Clone)]
pub struct StopToken {
    flag: Arc<AtomicBool>,
}

impl StopToken {
    pub fn new() -> Self {
        Self {
            flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn stop(&self) {
        self.flag.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn should_stop(&self) -> bool {
        self.flag.load(std::sync::atomic::Ordering::SeqCst)
    }
}

pub trait Engine: Clone + Send + Sync {
    fn name(&self) -> String;

    fn author(&self) -> String;

    fn search(&self, board: &Board, params: SearchParams, stop: StopToken);
}

#[derive(Clone)]
pub struct TestEngine {}

impl Engine for TestEngine {
    fn name(&self) -> String {
        String::from("Test engine")
    }

    fn author(&self) -> String {
        String::from("Tester")
    }

    fn search(&self, board: &Board, params: SearchParams, stop: StopToken) {
        let start_time = Instant::now();
        let our_milliseconds_left = match board.color_to_move() {
            Color::White => params.wtime,
            Color::Black => params.btime,
        };

        let time_to_think = our_milliseconds_left as u128; // think for x milliseconds

        let mut max = -INF;
        let mut bestmove = Move::null();

        let mut depth = 1;
        while (Instant::now() - start_time).as_millis() < time_to_think {
            let moves = generate_moves(board);
            println!(
                "Depth: {}, elapsed: {}",
                depth,
                (Instant::now() - start_time).as_millis()
            );

            let mut inner_max = -INF;
            let mut inner_bestmove = Move::null();

            for mv in &moves {
                if stop.should_stop()
                    || ((Instant::now() - start_time).as_millis() >= time_to_think)
                {
                    break;
                }
                let score = -negamax_alphabeta(&board.do_move(mv), -INF, INF, 1, depth - 1);

                // println!("{}: {}", mv, score);
                if score > inner_max {
                    inner_max = score;
                    inner_bestmove = mv;
                }
            }

            if inner_max > max {
                max = inner_max;
                bestmove = inner_bestmove;
            }
            depth += 1;
        }

        println!("bestmove {}, score {}", bestmove, max);
        println!(
            "Time elapsed: {}({})",
            (Instant::now() - start_time).as_millis(),
            time_to_think
        );
    }
}

pub struct UCIEngine {}

impl UCIEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self, engine: Arc<impl Engine + 'static>) {
        let stdin = std::io::stdin();

        let mut stop_token: Option<StopToken> = None;
        let mut board = Board::default();

        for line in stdin.lock().lines() {
            let line = line.unwrap();

            match uci::parse_command(&line) {
                Ok(command) => match command {
                    UCICommand::Quit => break,
                    UCICommand::Stop => match stop_token.take() {
                        Some(stop) => stop.stop(),
                        _ => (),
                    },
                    UCICommand::Display => eprintln!("{}", board),
                    UCICommand::Uci => {
                        println!("id name {}", engine.name());
                        println!("id author {}", engine.author());
                        println!("uciok");
                    }
                    UCICommand::IsReady => println!("readyok"), // TODO: check if actually ready
                    UCICommand::Position(fen, moves) => match Board::from_fen(&fen) {
                        Ok(b) => {
                            board = b;
                            for mv in moves {
                                match uci::parse_move(&mv, &board) {
                                    Ok(m) => board = board.do_move(m),
                                    _ => {
                                        eprintln!("Invalid move");
                                        break;
                                    }
                                }
                            }
                        }
                        Err(e) => eprintln!("{}", e),
                    },
                    UCICommand::Go(params) => {
                        let stop = StopToken::new();
                        stop_token = Some(stop.clone());

                        // TODO: Must the handles be joined??
                        let _ = std::thread::spawn({
                            let stop_clone = stop.clone();
                            let board_clone = board.clone();
                            let engine_clone = Arc::clone(&engine);
                            move || {
                                engine_clone.search(&board_clone, params, stop_clone);
                            }
                        });
                    }
                    UCICommand::Perft(depth) => {
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
                        eprintln!(
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
}

fn main() {
    let engine = Arc::new(TestEngine {});
    let mut uci = UCIEngine::new();

    uci.run(Arc::clone(&engine));
}
