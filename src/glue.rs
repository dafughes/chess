use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    board::Board,
    moves::{self, generate_moves, MoveKind},
    piece::{Piece, PieceKind},
    square::Square,
};

use rand::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub promotion_type: u8,
}

#[wasm_bindgen]
impl Move {
    #[wasm_bindgen(constructor)]
    pub fn new(from: u8, to: u8, promotion_type: u8) -> Self {
        Self {
            from,
            to,
            promotion_type,
        }
    }

    pub(crate) fn from_internal_move(mv: moves::Move) -> Self {
        let from = mv.from().to_index() as u8;
        let to = mv.to().to_index() as u8;
        let promotion_type = match mv.kind() {
            MoveKind::Prom(piece) | MoveKind::PromCap(piece) => piece.to_index() as u8,
            _ => 5,
        };

        Self {
            from,
            to,
            promotion_type,
        }
    }

    pub(crate) fn promotion_piece(&self) -> Option<PieceKind> {
        match self.promotion_type {
            1 => Some(PieceKind::Knight),
            2 => Some(PieceKind::Bishop),
            3 => Some(PieceKind::Rook),
            4 => Some(PieceKind::Queen),
            _ => None,
        }
    }

    pub(crate) fn to_internal_move(&self, board: &Board) -> Option<moves::Move> {
        let from = Square::from_index(self.from as usize);
        let to = Square::from_index(self.to as usize);

        for mv in &generate_moves(board) {
            if mv.from() == from && mv.to() == to {
                if let Some(piece) = self.promotion_piece() {
                    match mv.kind() {
                        MoveKind::Prom(p) | MoveKind::PromCap(p) => {
                            if piece == p {
                                return Some(mv);
                            }
                        }
                        _ => (),
                    }
                } else {
                    return Some(mv);
                }
            }
        }

        None
    }
}

#[wasm_bindgen]
pub struct Game {
    board: Board,
    rng: ThreadRng,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            rng: thread_rng(),
        }
    }

    pub fn piece_at(&self, square: u8) -> u8 {
        let square = Square::from_index(square as usize);
        match self.board.at(square) {
            Some(piece) => piece.kind().to_index() as u8 + piece.color().to_index() as u8 * 8,

            None => 16,
        }
    }

    pub fn make_move(&mut self, mv: &Move) {
        if let Some(mv) = mv.to_internal_move(&self.board) {
            self.board = self.board.do_move(mv);
        }
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        generate_moves(&self.board)
            .into_iter()
            .map(|mv| Move::from_internal_move(mv))
            .collect()
    }

    pub fn best_move(&mut self) -> Move {
        let moves = generate_moves(&self.board);

        moves
            .into_iter()
            .choose(&mut self.rng)
            .map(|m| Move::from_internal_move(m))
            .unwrap()
    }
}
