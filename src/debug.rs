use std::fmt;

use crate::{bitboard::Bitboard, board::Board, moves::generate_moves, square::Square};

pub fn perft(board: &Board, depth: usize) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = generate_moves(board);

    if depth == 1 {
        return moves.count() as u64;
    }

    let mut count = 0;

    for mv in &moves {
        count += perft(&board.do_move(mv), depth - 1);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{io::BufRead, time::Instant};

    #[test]
    fn perft_test() {
        let f = std::fs::File::open("perftsuite.txt").unwrap();

        let reader = std::io::BufReader::new(f);

        let t = Instant::now();
        let mut total_nodes = 0;

        for line in reader.lines() {
            let line = line.unwrap();

            let mut tokens = line.split(',');
            let fen = tokens.next().unwrap();
            let mut board = Board::from_fen(fen);

            print!("FEN: {: <80}|", fen);

            for (depth, token) in tokens.enumerate() {
                let actual_nodes = token.trim().parse::<u64>().unwrap();
                let nodes = perft(&mut board, depth + 1);
                total_nodes += nodes;

                assert_eq!(nodes, actual_nodes, "FEN: {}, depth={}", fen, depth + 1);
                print!(" {}", nodes);
            }
            println!("");
        }

        let elapsed = (Instant::now() - t).as_secs_f64();
        let nps = total_nodes as f64 / elapsed;

        let (nps, prefix) = if nps > 1e6 {
            (nps / 1e6, "Mnps")
        } else if nps > 1e3 {
            (nps / 1e3, "knps")
        } else {
            (nps, "nps")
        };

        println!(
            "Leaf nodes searched: {}, elapsed time: {:.2} s ({:.2} {})",
            total_nodes, elapsed, nps, prefix
        );
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "   | a | b | c | d | e | f | g | h |")?;
        writeln!(f, "---+---+---+---+---+---+---+---+---+---")?;

        for rank in (0..8).rev() {
            write!(f, " {} |", rank + 1)?;
            for file in 0..8 {
                let square = Square::from_index(rank * 8 + file);

                let c = match self.at(square) {
                    Some(piece) => char::from(piece),
                    None => ' ',
                };

                write!(f, " {} |", c)?;
            }
            writeln!(f, " {}", rank + 1)?;
            writeln!(f, "---+---+---+---+---+---+---+---+---+---")?;
        }

        writeln!(f, "   | a | b | c | d | e | f | g | h |\n")?;
        writeln!(f, "FEN: {}", self.fen())
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "   | a | b | c | d | e | f | g | h |")?;
        writeln!(f, "---+---+---+---+---+---+---+---+---+---")?;

        for rank in (0..8).rev() {
            write!(f, " {} |", rank + 1)?;
            for file in 0..8 {
                let square = Square::from_index(rank * 8 + file);

                let c = if (*self & square).is_non_empty() {
                    'X'
                } else {
                    ' '
                };

                write!(f, " {} |", c)?;
            }
            writeln!(f, " {}", rank + 1)?;
            writeln!(f, "---+---+---+---+---+---+---+---+---+---")?;
        }

        writeln!(f, "   | a | b | c | d | e | f | g | h |")
    }
}
