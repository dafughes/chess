use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use crate::{
    eval::{material, piece_positions, Score},
    game::{
        color::Color,
        moves::{Move, MoveKind, Movelist},
        Game,
    },
    uci::SearchParams,
};

fn evaluate(game: &Game) -> Score {
    material(game) + piece_positions(game)
}

fn order_moves(game: &Game, moves: &mut Movelist) {
    // Order captures based on most valuable victim - least valuable aggressor
    // Indexing: [victim][aggressor]
    const MVV_LVA: [[u8; 6]; 6] = [
        [1, 2, 3, 4, 5, 6], // Victim: Pawn, Aggressors: [P, N, B, R, Q, K, None]
        [7, 8, 9, 10, 11, 12],
        [13, 14, 15, 16, 17, 18],
        [19, 20, 21, 22, 23, 24],
        [25, 26, 27, 28, 29, 30],
        [31, 32, 33, 34, 35, 36],
    ];

    fn rate_capture(mv: &Move, game: &Game) -> u8 {
        match mv.kind() {
            MoveKind::EnPassant => 1,
            MoveKind::Capture | MoveKind::PromotionCapture(_) => {
                let aggressor = game.get_piece(mv.from()).unwrap();
                let victim = game.get_piece(mv.to()).unwrap();
                MVV_LVA[victim.kind() as usize][aggressor.kind() as usize]
            }
            _ => 0,
        }
    }

    moves.sort_by(|a, b| rate_capture(b, game).cmp(&rate_capture(a, game)));
}

pub fn search(mut game: Game, params: SearchParams, stop: Arc<AtomicBool>) -> (Move, Score) {
    let mut nodes_searched = 0;

    if params.depth > 0 {
        search_depth(&mut game, params.depth, &params, &stop, &mut nodes_searched)
    } else {
        const MAX_DEPTH: usize = 16;

        // Time management
        let average_moves_per_game = 70;
        let moves_left = average_moves_per_game - game.fullmove_number();

        let time_left_ms = match game.color_to_move() {
            Color::White => params.wtime,
            Color::Black => params.btime,
        };

        let search_time_ms = Duration::from_millis(time_left_ms as u64 / moves_left as u64);
        println!("{:?}", search_time_ms);
        let start_time = Instant::now();

        let mut best_move: Option<Move> = None;
        let mut best_score = Score::Min;

        let mut prev_start = Instant::now();
        let mut prev_time = Duration::from_millis(0);

        for depth in 1..=MAX_DEPTH {
            let average_branching_factor = 30;
            let predicted_search_time = prev_time * average_branching_factor;

            if (Instant::now() - prev_start) + predicted_search_time
                > Duration::from_secs_f64(search_time_ms.as_secs_f64() * 1.5)
            {
                // Don't start a new search if it takes too much time
                println!(
                    "{:?}, {:?}",
                    Instant::now() - start_time,
                    predicted_search_time
                );
                break;
            }

            // Check time/stop
            if stop.load(Ordering::Relaxed) {
                break;
            }

            let (mv, score) = search_depth(&mut game, depth, &params, &stop, &mut nodes_searched);
            prev_time = Instant::now() - prev_start;
            prev_start = Instant::now();

            let score = -score;

            best_move = Some(mv);
            best_score = score;

            println!(
                "info depth {} score {} nodes {}",
                depth, score, nodes_searched
            );
        }

        (best_move.unwrap_or_default(), best_score)
    }
}

pub fn search_depth(
    game: &mut Game,
    max_depth: usize,
    params: &SearchParams,
    stop: &Arc<AtomicBool>,
    nodes_searched: &mut u64,
) -> (Move, Score) {
    let mut moves = game.moves();

    order_moves(game, &mut moves);

    let mut bestmove: Option<Move> = None;
    let mut alpha = Score::Min;
    let beta = Score::Max;

    let color = game.color_to_move();

    for mv in moves {
        if stop.load(Ordering::Relaxed) {
            break;
        }

        game.push(mv);
        let score = -negamax_ab(game, -beta, -alpha, 1, max_depth, !color, nodes_searched);
        game.pop();

        if score > alpha {
            alpha = score;
            bestmove = Some(mv);
        }

        if score >= beta {
            break;
        }
    }

    if let Some(mv) = bestmove {
        (mv, alpha)
    } else {
        (Move::default(), Score::default())
    }
}

fn negamax_ab(
    game: &mut Game,
    mut alpha: Score,
    beta: Score,
    depth: usize,
    max_depth: usize,
    color: Color,
    nodes_searched: &mut u64,
) -> Score {
    *nodes_searched += 1;

    let mut moves = game.moves();
    order_moves(game, &mut moves);

    if moves.is_empty() {
        if game.is_check() {
            return -Score::mate(depth);
        } else {
            return Score::Draw;
        }
    } else if depth >= max_depth {
        return quiescence(game, alpha, beta, depth, color, nodes_searched);
    }

    if game.repetition_count() >= 3 || game.fifty_move_counter() >= 100 {
        return Score::Draw;
    }

    for mv in moves {
        game.push(mv);
        let score = -negamax_ab(
            game,
            -beta,
            -alpha,
            depth + 1,
            max_depth,
            !color,
            nodes_searched,
        );
        game.pop();

        alpha = alpha.max(score);

        if score >= beta {
            break;
        }
    }

    alpha
}

fn quiescence(
    game: &mut Game,
    mut alpha: Score,
    beta: Score,
    depth: usize,
    color: Color,
    nodes_searched: &mut u64,
) -> Score {
    *nodes_searched += 1;

    let mut moves = game.moves();

    moves.filter(|mv| mv.is_cap());

    order_moves(game, &mut moves);

    let standing_pat = match color {
        Color::White => evaluate(game),
        Color::Black => -evaluate(game),
    };

    let mut best = standing_pat;

    if standing_pat >= beta {
        return standing_pat;
    }

    alpha = alpha.max(standing_pat);

    if moves.is_empty() {
        if game.is_check() {
            return -Score::mate(depth);
        } else {
            return Score::Draw;
        }
    }

    if game.repetition_count() >= 3 || game.fifty_move_counter() >= 100 {
        return Score::Draw;
    }

    for mv in moves {
        game.push(mv);
        let score = -quiescence(game, -beta, -alpha, depth + 1, !color, nodes_searched);
        game.pop();

        if score >= beta {
            return score;
        }

        best = best.max(score);
        alpha = alpha.max(score);
    }

    best
}

#[cfg(test)]
mod tests {
    use std::sync::{atomic::AtomicBool, Arc};

    use crate::{
        game::Game,
        search::search_depth,
        uci::{self, SearchParams},
    };

    #[test]
    fn mate_in_2() {
        let test_positions = [
            (
                "r2qk2r/pb4pp/1n2Pb2/2B2Q2/p1p5/2P5/2B2PPP/RN2R1K1 w - -",
                "f5g6",
            ),
            ("6k1/pp4p1/2p5/2bp4/8/P5Pb/1P3rrP/2BRRN1K b - -", "g2g1"),
            ("8/2k2p2/2b3p1/P1p1Np2/1p3b2/1P1K4/5r2/R3R3 b - -", "c6b5"),
            ("6k1/5p2/1p5p/p4Np1/5q2/Q6P/PPr5/3R3K w - -", "a3f8"),
            (
                "r1b2k1r/ppppq3/5N1p/4P2Q/4PP2/1B6/PP5P/n2K2R1 w - -",
                "h5h6",
            ),
        ];

        for (fen, mv) in test_positions {
            let mut game = Game::new(fen).unwrap();
            assert_eq!(
                uci::parse_move(&mut game, mv).unwrap(),
                search_depth(
                    &mut game,
                    3,
                    &SearchParams::new(),
                    &Arc::new(AtomicBool::new(false)),
                    &mut 0
                )
                .0,
                "FEN: {fen}"
            );
        }
    }

    #[test]
    fn mate_in_3a() {
        let fen = "2r3k1/p4p2/3Rp2p/1p2P1pK/8/1P4P1/P3Q2P/1q6 b - -";
        let mut game = Game::new(fen).unwrap();
        let mv = uci::parse_move(&mut game, "b1g6").unwrap();
        assert_eq!(
            search_depth(
                &mut game,
                5,
                &SearchParams::new(),
                &Arc::new(AtomicBool::new(false)),
                &mut 0
            )
            .0,
            mv,
            "FEN: {}",
            fen
        );
    }

    #[test]
    fn mate_in_3b() {
        let fen = "1k5r/pP3ppp/3p2b1/1BN1n3/1Q2P3/P1B5/KP3P1P/7q w - -";

        let mut game = Game::new(fen).unwrap();
        let mv = uci::parse_move(&mut game, "c5a6").unwrap();
        assert_eq!(
            search_depth(
                &mut game,
                5,
                &SearchParams::new(),
                &Arc::new(AtomicBool::new(false)),
                &mut 0
            )
            .0,
            mv,
            "FEN: {}",
            fen
        );
    }

    #[test]
    fn mate_in_3c() {
        let fen = "3r4/pR2N3/2pkb3/5p2/8/2B5/qP3PPP/4R1K1 w - -";
        let mut game = Game::new(fen).unwrap();
        let mv = uci::parse_move(&mut game, "c3e5").unwrap();
        assert_eq!(
            search_depth(
                &mut game,
                5,
                &SearchParams::new(),
                &Arc::new(AtomicBool::new(false)),
                &mut 0
            )
            .0,
            mv,
            "FEN: {}",
            fen
        );
    }

    #[test]
    fn mate_in_3d() {
        let fen = "R6R/1r3pp1/4p1kp/3pP3/1r2qPP1/7P/1P1Q3K/8 w - -";
        let mut game = Game::new(fen).unwrap();
        let mv = uci::parse_move(&mut game, "f4f5").unwrap();
        assert_eq!(
            search_depth(
                &mut game,
                5,
                &SearchParams::new(),
                &Arc::new(AtomicBool::new(false)),
                &mut 0
            )
            .0,
            mv,
            "FEN: {}",
            fen
        );
    }

    #[test]
    fn mate_in_4() {
        let fen = "8/k2r4/p7/2b1Bp2/P3p3/qp4R1/4QP2/1K6 b - - 0 1";
        let mut game = Game::new(fen).unwrap();
        let mv = uci::parse_move(&mut game, "d7d1").unwrap();
        assert_eq!(
            search_depth(
                &mut game,
                7,
                &SearchParams::new(),
                &Arc::new(AtomicBool::new(false)),
                &mut 0
            )
            .0,
            mv,
            "FEN: {}",
            fen
        );
    }
}
