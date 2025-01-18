use crate::board::{color::Color, moves::Move, Board};

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
