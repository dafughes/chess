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
    // engine::main_loop::<Engine>();

    // let mut params = SearchParams::new();
    // params.depth = Some(5);

    // let pos = Board::from_str("3r4/pR2N3/2pkb3/5p2/8/2B5/qP3PPP/4R1K1 w - - 1 0").unwrap();

    // let mut search = Search::new(
    //     pos.clone(),
    //     vec![],
    //     Arc::new(AtomicBool::new(false)),
    //     params,
    // );
    // let mut nodes = 0;
    // let mut mv = Move::default();
    // let t = Timer::new();
    // for i in 0..10 {
    //     mv = search.search_alphabeta();
    //     nodes += search.leaf_nodes_searched;
    // }

    // let elapsed = t.elapsed();
    // let nps = nodes as f64 / elapsed;

    // println!(
    //     "Bestmove: {}, nodes searched: {}, Elapsed: {} s, nps: {:.2}",
    //     mv,
    //     nodes,
    //     t.elapsed(),
    //     nps
    // );
}
