use std::{io::BufRead, time::Instant};

use chess::board::{
    moves::{perft, perft_divide},
    Board,
};

pub mod uci;

fn main() {
    let stdin = std::io::stdin();

    let mut board = Board::default();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        match line.parse::<uci::Command>() {
            Ok(uci::Command::Quit) => break,
            Ok(uci::Command::Display) => println!("{}", board),
            Ok(uci::Command::Perft(depth)) => {
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
            Ok(uci::Command::Position(fen, moves)) => {
                if let Ok(mut b) = fen.parse() {
                    for m in moves {
                        if let Ok(mv) = uci::parse_move(&b, &m) {
                            b = b.do_move(mv);
                        } else {
                            b = Board::default();
                            break;
                        }
                    }

                    board = b;
                } else {
                    board = Board::default();
                }
            }
            Ok(command) => println!("{:?}", command),
            Err(e) => println!("{}", e),
        }
    }
}
