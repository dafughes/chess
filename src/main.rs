use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Instant,
};

use chess::{
    board::{moves::Move, Board},
    engine::{self, ChessEngine},
    search::Search,
    uci::SearchParams,
};

pub struct Engine;

impl ChessEngine for Engine {
    fn name() -> String {
        String::from("Engine v5")
    }

    fn author() -> String {
        String::from("Sam")
    }

    fn search(
        startpos: Board,
        moves_played: Vec<Move>,
        params: SearchParams,
        stop: Arc<AtomicBool>,
    ) {
        let mut search = Search::new(startpos, moves_played, stop, params);

        let bestmove = search.search_alphabeta();
        println!("bestmove {}", bestmove);
    }
}

pub struct Timer {
    t: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self { t: Instant::now() }
    }

    pub fn elapsed(&self) -> f64 {
        (Instant::now() - self.t).as_secs_f64()
    }
}

fn main() {
    engine::main_loop::<Engine>();
}
