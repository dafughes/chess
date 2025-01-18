use std::sync::{atomic::AtomicBool, Arc};

use chess::{board::Board, uci};
use rand::seq::IteratorRandom;

use super::ChessEngine;

pub struct RandomEngine;

impl ChessEngine for RandomEngine {
    fn name() -> String {
        String::from("Random")
    }

    fn author() -> String {
        String::from("Sam")
    }

    fn search(board: Board, _: Vec<Board>, _: uci::SearchParams, _: Arc<AtomicBool>) {
        let bestmove = board
            .moves()
            .into_iter()
            .choose(&mut rand::thread_rng())
            .unwrap_or_default();

        println!("bestmove {}", bestmove);
    }
}
