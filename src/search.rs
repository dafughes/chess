use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use crate::{
    board::{
        color::Color,
        moves::{Move, MoveKind, Movelist},
        piece::PieceKind,
        Board,
    },
    eval::{material, piece_positions},
    uci::SearchParams,
    value::{MoveWithValue, Value},
};

pub struct StackStack<T, const N: usize> {
    data: [T; N],
    top: usize,
}

impl<T: Default + Copy, const N: usize> StackStack<T, N> {
    pub fn new() -> Self {
        Self {
            data: [T::default(); N],
            top: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        self.data[self.top] = value;
        self.top += 1;
    }

    pub fn pop(&mut self) {
        if self.top > 0 {
            self.top -= 1;
        }
    }

    pub fn clear(&mut self) {
        self.top = 0;
    }

    pub fn len(&self) -> usize {
        self.top
    }
}

pub struct IntoIter<'a, T, const N: usize> {
    stack: &'a StackStack<T, N>,
    i: usize,
}

impl<'a, T: Copy, const N: usize> Iterator for IntoIter<'a, T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.stack.top {
            self.i += 1;
            Some(self.stack.data[self.i - 1])
        } else {
            None
        }
    }
}

impl<'a, T: Copy, const N: usize> IntoIterator for &'a StackStack<T, N> {
    type Item = T;
    type IntoIter = IntoIter<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { stack: self, i: 0 }
    }
}

pub struct Search {
    root: Board,
    stop: Arc<AtomicBool>,
    params: SearchParams,
    nodes_searched: u64,
    history: StackStack<u64, 1024>,
    start_time: Instant,
}

impl Search {
    pub fn new(
        startpos: Board,
        moves: Vec<Move>,
        stop: Arc<AtomicBool>,
        params: SearchParams,
    ) -> Self {
        let mut root = startpos;

        let mut history = StackStack::new();
        history.push(root.hash());

        for mv in moves {
            // Captures or pawn moves are irreversible, so history can start from those
            root = root.do_move(mv);
            if root.halfmove_clock() == 0 {
                history.clear();
            }
            history.push(root.hash());
        }

        Self {
            root,
            stop,
            params,
            nodes_searched: 0,
            history,
            start_time: Instant::now(),
        }
    }

    fn info(&self, depth: usize, currmove: Move, score: Value, t: Duration) {
        let nps = self.nodes_searched as f64 / t.as_secs_f64();
        let time = Instant::now() - self.start_time;
        println!(
            "info depth {} score {} nps {} currmove {} nodes {} time {}",
            depth,
            score,
            nps as u64,
            currmove,
            self.nodes_searched,
            time.as_millis()
        );
    }

    pub fn order_moves(board: &Board, moves: &mut Movelist) {
        // Order captures based on most valuable victim - least valuable aggressor
        // Indexing: [victim][aggressor]
        const mvv_lva: [[u8; 6]; 6] = [
            [1, 2, 3, 4, 5, 6], // Victim: Pawn, Aggressors: [P, N, B, R, Q, K, None]
            [7, 8, 9, 10, 11, 12],
            [13, 14, 15, 16, 17, 18],
            [19, 20, 21, 22, 23, 24],
            [25, 26, 27, 28, 29, 30],
            [31, 32, 33, 34, 35, 36],
        ];

        fn rate_capture(mv: &Move, board: &Board) -> u8 {
            match mv.kind() {
                MoveKind::EnPassant => 1,
                MoveKind::Capture | MoveKind::PromotionCapture(_) => {
                    let aggressor = board.get(mv.from()).unwrap();
                    let victim = board.get(mv.to()).unwrap();
                    mvv_lva[victim.kind() as usize][aggressor.kind() as usize]
                }
                _ => 0,
            }
        }

        moves.sort_by(|a, b| rate_capture(b, board).cmp(&rate_capture(a, board)));
    }

    pub fn search_alphabeta(&mut self) -> Move {
        // Reset node counter, reset every depth iteration?
        self.nodes_searched = 0;

        // Calculate target search time
        let average_moves_per_game = 80;
        let past_moves = self.root.fullmove_number();
        let moves_to_go = average_moves_per_game - past_moves;

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

            let mut moves = self.root.moves();
            Self::order_moves(&self.root, &mut moves);

            for mv in moves {
                // Check time & stop command
                if (has_time_limit && ((Instant::now() - start_time) >= search_time))
                    || self.stop.load(Ordering::Relaxed)
                {
                    if let Some(best) = outer_best {
                        return best.mv;
                    } else {
                        return self.root.moves().into_iter().next().unwrap();
                    }
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
        if let Some(best) = outer_best {
            best.mv
        } else {
            self.root.moves().into_iter().next().unwrap()
        }
    }

    fn evaluate(&self, board: &Board) -> Value {
        Value::Evaluation(material(board) + piece_positions(board))
    }

    fn quiescence(
        &mut self,
        board: Board,
        mut alpha: Value,
        beta: Value,
        depth: usize,
        color: Color,
    ) -> Value {
        self.nodes_searched += 1;

        let mut moves = board.moves();

        let standing_pat = match color {
            Color::White => self.evaluate(&board),
            Color::Black => -self.evaluate(&board),
        };
        let mut best = standing_pat;
        if standing_pat >= beta {
            return standing_pat;
        }
        if standing_pat > alpha {
            alpha = standing_pat;
        }

        if moves.is_empty() {
            if board.in_check() {
                return -Value::mate(depth);
            } else {
                return Value::Draw;
            }
        }

        // Check repetitions
        if self
            .history
            .into_iter()
            .filter(|h| *h == board.hash())
            .count()
            >= 2
        {
            return Value::Draw;
        }

        // TODO: 50-move rule

        Self::order_moves(&board, &mut moves);

        for mv in moves {
            if !mv.is_cap() {
                continue;
            }
            self.history.push(board.hash());
            let value = -self.quiescence(board.do_move(mv), -beta, -alpha, depth + 1, !color);
            self.history.pop();

            if value >= beta {
                return value;
            }
            if value > best {
                best = value;
            }
            if value > alpha {
                alpha = value;
            }
        }

        best
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
        self.nodes_searched += 1;

        let mut moves = board.moves();

        if moves.is_empty() {
            if board.in_check() {
                return -Value::mate(depth);
            } else {
                return Value::Draw;
            }
        } else if depth >= max_depth {
            return self.quiescence(board, alpha, beta, depth, color);
            // self.leaf_nodes_searched += 1;
            // return match color {
            //     Color::White => self.evaluate(&board),
            //     Color::Black => -self.evaluate(&board),
            // };
        }

        // Check repetitions
        if self
            .history
            .into_iter()
            .filter(|h| *h == board.hash())
            .count()
            >= 2
        {
            return Value::Draw;
        }

        // TODO: 50-move rule

        Self::order_moves(&board, &mut moves);

        for mv in moves {
            self.history.push(board.hash());
            let value = -self.negamax_alphabeta(
                board.do_move(mv),
                -beta,
                -alpha,
                depth + 1,
                max_depth,
                !color,
            );
            self.history.pop();

            if value > alpha {
                alpha = value;
            }

            if value >= beta {
                return alpha;
            }
        }

        alpha
    }
}

#[cfg(test)]
mod tests {
    use std::{
        str::FromStr,
        sync::{atomic::AtomicBool, Arc},
    };

    use crate::{
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
