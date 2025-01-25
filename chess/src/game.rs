use chrono::Utc;

use crate::board::{
    color::Color,
    moves::{Move, MoveKind},
    piece::PieceKind,
    Board,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DrawKind {
    Stalemate,
    ThreefoldRepetition,
    FiftyMoveRule,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameResult {
    Mate(Color),
    TimeOut(Color),
    Draw(DrawKind),
    Ongoing,
}

pub struct Game {
    history: Vec<Board>,
    moves: Vec<Move>,
    result: GameResult,
}

impl From<Board> for Game {
    fn from(value: Board) -> Self {
        let result = Self::calculate_result(&value, &[]);
        Self {
            history: vec![value],
            moves: vec![],
            result: result,
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            history: vec![Board::default()],
            moves: vec![],
            result: GameResult::Ongoing,
        }
    }

    fn calculate_result(board: &Board, history: &[Board]) -> GameResult {
        if board.moves().is_empty() {
            if board.in_check() {
                GameResult::Mate(!board.color_to_move())
            } else {
                GameResult::Draw(DrawKind::Stalemate)
            }
        } else {
            if board.halfmove_clock() >= 100 {
                GameResult::Draw(DrawKind::FiftyMoveRule)
            } else if history.iter().filter(|b| **b == *board).count() == 2 {
                GameResult::Draw(DrawKind::ThreefoldRepetition)
            } else {
                GameResult::Ongoing
            }
            // TODO! Insufficient material
        }
    }

    pub fn do_move(&mut self, mv: Move) {
        if self.result == GameResult::Ongoing {
            let board = self.board().do_move(mv);

            self.result = Self::calculate_result(&board, &self.history);

            self.history.push(board);
            self.moves.push(mv);
        }
    }

    pub fn result(&self) -> GameResult {
        self.result
    }

    pub fn board(&self) -> &Board {
        self.history.last().unwrap()
    }

    pub fn moves(&self) -> &[Move] {
        &self.moves
    }

    /// Used for setting result manually, e.g. time out
    pub fn set_result(&mut self, result: GameResult) {
        self.result = result;
    }
}

fn move_as_san(mv: &Move, board: &Board) -> String {
    // Piece symbol
    let piece_kind = board.get(mv.from()).unwrap().kind();
    let piece_symbol = match piece_kind {
        PieceKind::Pawn => String::new(),
        _ => char::from(piece_kind).to_ascii_uppercase().to_string(),
    };

    // Square
    let to = mv.to().to_string();

    // Count ambiguity
    let ambiguous_moves = board
        .moves()
        .into_iter()
        .filter(|m| m.to() == mv.to() && board.get(m.from()).unwrap().kind() == piece_kind)
        .collect::<Vec<_>>();

    let from: String = if ambiguous_moves.len() == 1 {
        String::new()
    } else {
        let file_ambiguity = ambiguous_moves
            .iter()
            .filter(|m| m.from().file() == mv.from().file())
            .count();
        let rank_ambiguity = ambiguous_moves
            .iter()
            .filter(|m| m.from().rank() == mv.from().rank())
            .count();

        if file_ambiguity == 1 {
            mv.from().file().to_string()
        } else if rank_ambiguity == 1 {
            mv.from().rank().to_string()
        } else {
            mv.from().to_string()
        }
    };

    let mut san = match mv.kind() {
        MoveKind::CastleKingside => String::from("O-O"),
        MoveKind::CastleQueenside => String::from("O-O-O"),
        MoveKind::Capture | MoveKind::EnPassant => format!("{}{}x{}", piece_symbol, from, to),
        MoveKind::Promotion(prom) => format!(
            "{}{}{}={}",
            piece_symbol,
            from,
            to,
            char::from(prom).to_ascii_uppercase()
        ),
        MoveKind::PromotionCapture(prom) => format!(
            "{}x{}{}={}",
            piece_symbol,
            from,
            to,
            char::from(prom).to_ascii_uppercase()
        ),
        _ => format!("{}{}{}", piece_symbol, from, to),
    };

    // Check/Mate
    let new_board = board.do_move(*mv);
    if new_board.in_check() {
        if new_board.moves().is_empty() {
            san.push('#');
        } else {
            san.push('+');
        }
    }
    san
}

pub fn write_pgn(
    game: &Game,
    event: &str,
    site: &str,
    date: chrono::DateTime<Utc>,
    time_control: &str,
    round: &str,
    white: &str,
    black: &str,
) -> String {
    let mut pgn = String::new();

    pgn.push_str(format!("[Event \"{}\"]\n", event).as_str());
    pgn.push_str(format!("[Site \"{}\"]\n", site).as_str());
    pgn.push_str(format!("[Date \"{}\"]\n", date.format("%Y.%m.%d")).as_str());
    pgn.push_str(format!("[TimeControl \"{}\"]\n", time_control).as_str());
    pgn.push_str(format!("[Round \"{}\"]\n", round).as_str());
    let termination = match game.result() {
        GameResult::TimeOut(_) => "time forfeit",
        GameResult::Ongoing => "unterminated",
        _ => "normal",
    };
    pgn.push_str(format!("[Termination \"{}\"]\n", termination).as_str());
    pgn.push_str(format!("[White \"{}\"]\n", white).as_str());
    pgn.push_str(format!("[Black \"{}\"]\n", black).as_str());

    let result_string = match game.result() {
        GameResult::Mate(Color::White) | GameResult::TimeOut(Color::White) => "1 - 0",
        GameResult::Mate(Color::Black) | GameResult::TimeOut(Color::Black) => "0 - 1",
        GameResult::Draw(_) => "1/2-1/2",
        GameResult::Ongoing => "*",
    };
    pgn.push_str(format!("[Result \"{}\"]\n", result_string).as_str());

    pgn.push('\n');

    for (mv, board) in game.moves().iter().zip(game.history.iter()) {
        if board.color_to_move() == Color::Black {
            pgn.push_str(format!("{} ", move_as_san(mv, board)).as_str());
            pgn.push('\n');
        } else {
            pgn.push_str(format!("{}.", board.fullmove_number()).as_str());
            pgn.push_str(format!("{} ", move_as_san(mv, board)).as_str());
        }
    }

    pgn
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{
            color::Color,
            moves::{Move, MoveKind},
            square::Square,
            Board,
        },
        game::{DrawKind, GameResult},
    };

    use super::Game;

    #[test]
    fn ongoing() {
        let mut game = Game::new();

        game.do_move(Move::new(Square::A2, Square::A4, MoveKind::DoublePush));
        game.do_move(Move::new(Square::A7, Square::A5, MoveKind::DoublePush));
        assert_eq!(game.result(), GameResult::Ongoing);
    }

    #[test]
    fn stalemate() {
        let mut game = Game::from("1k6/8/K7/8/Q7/8/8/8 w - -".parse::<Board>().unwrap());

        assert_eq!(game.result(), GameResult::Ongoing);
        game.do_move(Move::new(Square::A4, Square::C6, MoveKind::Quiet));
        assert_eq!(game.result(), GameResult::Draw(DrawKind::Stalemate));
    }

    #[test]
    fn checkmate() {
        let mut game = Game::new();

        game.do_move(Move::new(Square::F2, Square::F3, MoveKind::Quiet));
        game.do_move(Move::new(Square::E7, Square::E5, MoveKind::DoublePush));
        game.do_move(Move::new(Square::G2, Square::G4, MoveKind::Quiet));
        game.do_move(Move::new(Square::D8, Square::H4, MoveKind::Quiet));
        assert_eq!(game.result(), GameResult::Mate(Color::Black));
    }

    #[test]
    fn repetition() {
        let mut game = Game::new();

        game.do_move(Move::new(Square::B1, Square::A3, MoveKind::Quiet));
        game.do_move(Move::new(Square::B8, Square::A6, MoveKind::Quiet));
        game.do_move(Move::new(Square::A3, Square::B1, MoveKind::Quiet));
        game.do_move(Move::new(Square::A6, Square::B8, MoveKind::Quiet));

        assert_eq!(game.result(), GameResult::Ongoing);

        game.do_move(Move::new(Square::B1, Square::A3, MoveKind::Quiet));
        game.do_move(Move::new(Square::B8, Square::A6, MoveKind::Quiet));
        game.do_move(Move::new(Square::A3, Square::B1, MoveKind::Quiet));
        game.do_move(Move::new(Square::A6, Square::B8, MoveKind::Quiet));

        assert_eq!(
            game.result(),
            GameResult::Draw(DrawKind::ThreefoldRepetition)
        );
    }
}
