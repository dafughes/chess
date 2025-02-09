use std::{
    io::BufRead,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::JoinHandle,
    time::Instant,
};

use crate::{
    game::{
        moves::{Move, MoveKind},
        perft_divide,
        square::Square,
        Game,
    },
    search::search,
};

pub struct SearchParams {
    pub wtime: u32,
    pub btime: u32,
    pub depth: usize,
}

impl SearchParams {
    pub fn new() -> Self {
        Self {
            wtime: 60000,
            btime: 60000,
            depth: 0,
        }
    }
}

const ENGINE_NAME: &str = "Engine";
const ENGINE_AUTHOR: &str = "Sam";

pub fn parse_position(command: &str) -> Option<Game> {
    let mut game = match command.split_ascii_whitespace().nth(1)? {
        "startpos" => Game::default(),
        "fen" => Game::new(
            command
                .split_ascii_whitespace()
                .skip(2)
                .take_while(|token| *token != "moves")
                .fold(String::new(), |a, b| a + " " + b)
                .trim(),
        )?,
        _ => return None,
    };

    if let Some((_, moves)) = command.split_once("moves") {
        for mv in moves.split_ascii_whitespace() {
            let mv = parse_move(&mut game, mv)?;
            game.push(mv);
        }
    }

    Some(game)
}

pub fn parse_go(command: &str) -> Option<SearchParams> {
    let mut params = SearchParams {
        wtime: 0,
        btime: 0,
        depth: 0,
    };

    let mut tokens = command.split_ascii_whitespace().skip(1);

    while let Some(t) = tokens.next() {
        match t {
            "wtime" => params.wtime = tokens.next()?.parse::<u32>().ok()?,
            "btime" => params.btime = tokens.next()?.parse::<u32>().ok()?,
            "depth" => params.depth = tokens.next()?.parse::<usize>().ok()?,
            _ => (),
        }
    }

    Some(params)
}

pub fn main_loop() {
    let stdin = std::io::stdin();

    let mut game = Game::default();
    let stop = Arc::new(AtomicBool::new(false));
    let mut search_handle: Option<JoinHandle<()>> = None;

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if let Some(command) = line.split_ascii_whitespace().next() {
            match command {
                "quit" => {
                    stop.store(true, Ordering::Relaxed);
                    break;
                }
                "stop" => {
                    stop.store(true, Ordering::Relaxed);
                }
                "uci" => {
                    println!("uciok");
                    println!("id name {ENGINE_NAME}");
                    println!("id author {ENGINE_AUTHOR}");
                }
                "ucinewgame" => {}
                "isready" => {
                    println!("readyok");
                }
                "position" => game = parse_position(line.as_str()).unwrap_or_default(),
                "go" => {
                    // Reset stop and wait for possible old search
                    stop.store(false, Ordering::Relaxed);
                    if let Some(h) = search_handle.take() {
                        h.join().unwrap();
                    }

                    if let Some(params) = parse_go(line.as_str()) {
                        search_handle = Some(std::thread::spawn({
                            let game_clone = game.clone();
                            let stop_clone = stop.clone();
                            || {
                                let (bestmove, _) = search(game_clone, params, stop_clone);
                                println!("bestmove {bestmove}");
                            }
                        }));
                    }
                }
                "d" => println!("{game}"),
                "perft" => {
                    let depth = line
                        .split_ascii_whitespace()
                        .skip(1)
                        .next()
                        .and_then(|d| d.parse::<usize>().ok())
                        .unwrap_or(1);

                    let t0 = Instant::now();
                    let nodes = perft_divide(&mut game, depth);
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
            }
        }
    }
    if let Some(h) = search_handle.take() {
        h.join().unwrap();
    }
}

pub fn parse_move(game: &mut Game, m: &str) -> Option<Move> {
    if m.len() < 4 || m.len() > 5 {
        return None;
    }

    let from = Square::new(
        m.chars().nth(1)?.try_into().ok()?,
        m.chars().nth(0)?.try_into().ok()?,
    );
    let to = Square::new(
        m.chars().nth(3)?.try_into().ok()?,
        m.chars().nth(2)?.try_into().ok()?,
    );

    let move_candidates = game
        .moves()
        .into_iter()
        .filter(|mv| mv.from() == from && mv.to() == to)
        .collect::<Vec<_>>();

    if move_candidates.is_empty() {
        None
    } else if move_candidates.len() == 1 {
        move_candidates.first().copied()
    } else {
        // Is a promotion
        let promotion_kind = m.chars().nth(4)?.try_into().ok()?;
        move_candidates
            .iter()
            .find(|mv| match mv.kind() {
                MoveKind::Promotion(kind) | MoveKind::PromotionCapture(kind) => {
                    kind == promotion_kind
                }
                _ => false,
            })
            .copied()
    }
}
