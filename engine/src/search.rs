use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use chess::{
    board::{color::Color, moves::Move, Board},
    uci::SearchParams,
};

use crate::{
    eval::material,
    value::{MoveWithValue, Value},
};

pub struct Search {
    root: Board,
    stop: Arc<AtomicBool>,
    params: SearchParams,
    pub leaf_nodes_searched: u64,
    history: [u64; 1024], // TODO: Max game length in plies?
    i: usize,
}

impl Search {
    pub fn new(
        startpos: Board,
        moves: Vec<Move>,
        stop: Arc<AtomicBool>,
        params: SearchParams,
    ) -> Self {
        // TODO: Build repetition history
        let mut root = startpos;
        let mut history = [0; 1024];
        history[0] = root.hash();
        let mut i = 1;

        for mv in moves {
            root = root.do_move(mv);
            history[i] = root.hash();
            i += 1;
        }

        Self {
            root,
            stop,
            params,
            leaf_nodes_searched: 0,
            history,
            i,
        }
    }

    fn info(&self, depth: usize, currmove: Move, score: Value, t: Duration) {
        let nps = self.leaf_nodes_searched as f64 / t.as_secs_f64();
        println!(
            "info depth {} score {} nps {} currmove {}",
            depth, score, nps as u64, currmove
        );
    }

    pub fn search(&mut self) -> Move {
        // Reset node counter, reset every depth iteration?
        self.leaf_nodes_searched = 0;

        // Calculate target search time
        let average_moves_per_game = 80;
        let past_moves = self.i;
        let moves_to_go = average_moves_per_game - past_moves / 2;

        // Search parameters should contain remaining time
        let time_remaining = match self.root.color_to_move() {
            Color::White => self.params.wtime.unwrap_or(60000),
            Color::Black => self.params.btime.unwrap_or(60000),
        };

        let search_time = time_remaining as f64 / moves_to_go as f64;

        let search_time = Duration::from_millis(search_time as u64);
        let start_time = Instant::now();

        let max_depth = 16;

        let mut outer_best: Option<MoveWithValue> = None;

        for depth in 1..max_depth {
            let mut best: Option<MoveWithValue> = None;

            for mv in self.root.moves() {
                // Check time & stop command
                if (Instant::now() - start_time) >= search_time || self.stop.load(Ordering::Relaxed)
                {
                    // TODO: If inner best is better, return it
                    return outer_best.unwrap_or_default().mv;
                }

                let new_board = self.root.do_move(mv);

                let value = -self.negamax(new_board, 1, depth);
                let mvw = MoveWithValue { mv, value };
                if let Some(b) = best {
                    if mvw > b {
                        best = Some(mvw);
                        self.info(depth, mv, value, Instant::now() - start_time);
                    }
                } else {
                    best = Some(mvw);
                    self.info(depth, mv, value, Instant::now() - start_time);
                }
            }

            outer_best = best;
        }

        outer_best.unwrap_or_default().mv
    }

    pub fn search_alphabeta(&mut self) -> Move {
        // Reset node counter, reset every depth iteration?
        self.leaf_nodes_searched = 0;

        // Calculate target search time
        let average_moves_per_game = 80;
        let past_moves = self.i;
        let moves_to_go = average_moves_per_game - past_moves / 2;

        // Search to depth
        let (max_depth, has_time_limit) = if let Some(depth) = self.params.depth {
            (depth, false)
        } else {
            (16, true)
        };

        // Search parameters should contain remaining time
        let time_remaining = match self.root.color_to_move() {
            Color::White => self.params.wtime.unwrap_or(60000),
            Color::Black => self.params.btime.unwrap_or(60000),
        };

        let search_time = time_remaining as f64 / moves_to_go as f64;

        let search_time = Duration::from_millis(search_time as u64);
        let start_time = Instant::now();

        let mut outer_best: Option<MoveWithValue> = None;

        for depth in 1..=max_depth {
            let mut bestmove: Option<Move> = None;
            let mut alpha = -Value::Evaluation(10000000);
            let beta = Value::Evaluation(10000000);

            for mv in self.root.moves() {
                // Check time & stop command
                if (has_time_limit && ((Instant::now() - start_time) >= search_time))
                    || self.stop.load(Ordering::Relaxed)
                {
                    return outer_best.unwrap_or_default().mv;
                }

                let new_board = self.root.do_move(mv);
                let color = new_board.color_to_move();

                let value = -self.negamax_alphabeta(new_board, -beta, -alpha, 1, depth, color);

                if value > alpha {
                    alpha = value;
                    self.info(depth, mv, value, Instant::now() - start_time);
                    bestmove = Some(mv);
                }

                if value >= beta {
                    break;
                }
            }

            outer_best = Some(MoveWithValue {
                mv: bestmove.unwrap_or_default(),
                value: alpha,
            });
        }
        outer_best.unwrap_or_default().mv
    }

    fn evaluate(&self, board: &Board) -> Value {
        Value::Evaluation(material(board))
        // Value::Evaluation(material(board) + piece_positions(board))
    }

    fn negamax_alphabeta(
        &mut self,
        board: Board,
        mut alpha: Value,
        beta: Value,
        depth: usize,
        max_depth: usize,
        color: Color,
    ) -> Value {
        // Update history
        self.history[self.i + depth - 1] = board.hash();

        let moves = board.moves();

        if moves.is_empty() {
            self.leaf_nodes_searched += 1;
            if board.in_check() {
                return -Value::mate(depth);
            } else {
                return Value::Draw;
            }
        } else if depth >= max_depth {
            self.leaf_nodes_searched += 1;
            return match color {
                Color::White => self.evaluate(&board),
                Color::Black => -self.evaluate(&board),
            };
        }

        // Check repetitions
        let mut rep = 0;
        for i in 0..(self.i + depth) {
            if self.history[i] == board.hash() {
                rep += 1;
            }
        }

        if rep >= 3 {
            return Value::Draw;
        }
        // if self.history[0..self.i]
        //     .iter()
        //     .filter(|h| **h == board.hash())
        //     .count()
        //     == 3
        // {
        //     return Value::Draw;
        // }

        // TODO: 50-move rule

        for mv in moves {
            let value = -self.negamax_alphabeta(
                board.do_move(mv),
                -beta,
                -alpha,
                depth + 1,
                max_depth,
                !color,
            );

            if value > alpha {
                alpha = value;
            }

            if value >= beta {
                return alpha;
            }
        }

        alpha
    }

    fn negamax(&mut self, board: Board, depth: usize, max_depth: usize) -> Value {
        let moves = board.moves();

        if moves.is_empty() {
            self.leaf_nodes_searched += 1;
            if board.in_check() {
                return -Value::mate(depth);
            } else {
                return Value::Draw;
            }
        } else if depth >= max_depth {
            self.leaf_nodes_searched += 1;
            return self.evaluate(&board);
        }

        let mut best: Option<Value> = None;

        for mv in moves {
            let value = -self.negamax(board.do_move(mv), depth + 1, max_depth);
            best = best.and_then(|b| Some(b.max(value))).or(Some(value));
        }

        best.unwrap_or_default()
    }

    fn minimize(&mut self, board: Board, depth: usize, max_depth: usize) -> Value {
        let moves = board.moves();

        if moves.is_empty() {
            if board.in_check() {
                return Value::mate(depth);
            } else {
                return Value::Draw;
            }
        } else if depth >= max_depth {
            return self.evaluate(&board);
        }

        let mut best: Option<Value> = None;

        for mv in moves {
            let value = self.maximize(board.do_move(mv), depth + 1, max_depth);
            best = best.and_then(|b| Some(b.min(value))).or(Some(value));
        }

        best.unwrap_or_default()
    }

    fn maximize(&mut self, board: Board, depth: usize, max_depth: usize) -> Value {
        let moves = board.moves();

        if moves.is_empty() {
            if board.in_check() {
                return -Value::mate(depth);
            } else {
                return Value::Draw;
            }
        } else if depth >= max_depth {
            return self.evaluate(&board);
        }

        let mut best: Option<Value> = None;

        for mv in moves {
            let value = self.minimize(board.do_move(mv), depth + 1, max_depth);
            best = best.and_then(|b| Some(b.max(value))).or(Some(value));
        }

        best.unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        str::FromStr,
        sync::{atomic::AtomicBool, Arc},
    };

    use chess::{
        board::{moves::Move, Board},
        uci::{self, SearchParams},
    };

    use super::Search;

    fn search(pos: &Board, depth: usize) -> Move {
        let mut params = SearchParams::new();
        params.depth = Some(depth);

        let mut search = Search::new(
            pos.clone(),
            vec![],
            Arc::new(AtomicBool::new(false)),
            params,
        );

        search.search_alphabeta()
    }

    #[test]
    fn mate_in_2() {
        let test_positions = [
            (
                "r2qk2r/pb4pp/1n2Pb2/2B2Q2/p1p5/2P5/2B2PPP/RN2R1K1 w",
                "f5g6",
            ),
            ("6k1/pp4p1/2p5/2bp4/8/P5Pb/1P3rrP/2BRRN1K b", "g2g1"),
            ("8/2k2p2/2b3p1/P1p1Np2/1p3b2/1P1K4/5r2/R3R3 b", "c6b5"),
            ("6k1/5p2/1p5p/p4Np1/5q2/Q6P/PPr5/3R3K w", "a3f8"),
            ("r1b2k1r/ppppq3/5N1p/4P2Q/4PP2/1B6/PP5P/n2K2R1 w ", "h5h6"),
        ];

        for (pos, mv) in test_positions {
            let board = Board::from_str(pos).unwrap();
            assert_eq!(
                search(&board, 3),
                uci::parse_move(&board, mv).unwrap(),
                "FEN: {pos}"
            );
        }
    }

    #[test]
    fn mate_in_3a() {
        let board = Board::from_str("2r3k1/p4p2/3Rp2p/1p2P1pK/8/1P4P1/P3Q2P/1q6 b").unwrap();
        let mv = uci::parse_move(&board, "b1g6").unwrap();
        assert_eq!(search(&board, 5), mv, "FEN: {}", board.fen());
    }

    #[test]
    fn mate_in_3b() {
        let board = Board::from_str("1k5r/pP3ppp/3p2b1/1BN1n3/1Q2P3/P1B5/KP3P1P/7q w").unwrap();
        let mv = uci::parse_move(&board, "c5a6").unwrap();
        assert_eq!(search(&board, 5), mv, "FEN: {}", board.fen());
    }

    #[test]
    fn mate_in_3c() {
        let board = Board::from_str("3r4/pR2N3/2pkb3/5p2/8/2B5/qP3PPP/4R1K1 w - -").unwrap();
        let mv = uci::parse_move(&board, "c3e5").unwrap();
        assert_eq!(search(&board, 5), mv, "FEN: {}", board.fen());
    }

    #[test]
    fn mate_in_3d() {
        let board = Board::from_str("R6R/1r3pp1/4p1kp/3pP3/1r2qPP1/7P/1P1Q3K/8 w").unwrap();
        let mv = uci::parse_move(&board, "f4f5").unwrap();
        assert_eq!(search(&board, 5), mv, "FEN: {}", board.fen());
    }
}
