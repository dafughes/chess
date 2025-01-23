use std::{
    collections::HashMap,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use chess::{
    board::{
        moves::{Move, MoveKind},
        square::Square,
        Board,
    },
    uci::SearchParams,
};
use engine::ChessEngine;
use rand::RngCore;
use search::Search;
use value::Value;

pub mod engine;
pub mod eval;
pub mod search;
pub mod value;

pub struct Engine;

impl ChessEngine for Engine {
    fn name() -> String {
        String::from("Engine v3")
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
