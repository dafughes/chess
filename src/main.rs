use chess::{
    eval::material,
    game::{castling_rights::CastlingRights, piece::Piece, square::Square, Game, State},
    uci::{self},
};

fn main() {
    // uci::main_loop();

    println!("{}", std::mem::size_of::<State>());
}
