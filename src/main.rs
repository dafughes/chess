use chess::{
    eval::material,
    game::{piece::Piece, square::Square, Game},
    uci::{self},
};

fn main() {
    uci::main_loop();

    // let mut game = Game::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -").unwrap();

    // game.put_piece(Piece::BlackKnight, Square::H8);
    // game.put_piece(Piece::WhiteRook, Square::H1);

    // println!("{}", material(&game));
}
