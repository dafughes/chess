use std::sync::{atomic::AtomicBool, Arc};

use chess::{
    eval::material,
    game::{castling_rights::CastlingRights, piece::Piece, square::Square, Game, State},
    search::search,
    uci::{self, parse_move, SearchParams},
};

fn main() {
    uci::main_loop();
}
