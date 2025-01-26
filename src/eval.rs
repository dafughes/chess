use crate::board::{
    color::Color,
    piece::PieceKind,
    square::{File, Rank, Square},
    Board,
};

// From https://www.chessprogramming.org/Simplified_Evaluation_Function
static PIECE_SQUARE_TABLES: [[i8; 64]; 6] = [
    // Pawn
    [
        0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5,
        5, 10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10,
        -20, -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    // Knight
    [
        -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15,
        10, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15,
        15, 10, 5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
    ],
    // Bishop
    [
        -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5,
        0, -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10,
        10, 10, -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
    ],
    // Rook
    [
        0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0,
        0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0,
        -5, 0, 0, 0, 5, 5, 0, 0, 0,
    ],
    // Queen
    [
        -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0,
        -10, -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0,
        5, 0, 0, 0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
    ],
    // King (opening, midgame)
    [
        -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40,
        -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40,
        -40, -30, -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20,
        30, 10, 0, 0, 10, 30, 20,
    ],
];

pub fn material(board: &Board) -> i32 {
    // Material value
    let white = board.pieces_by_color(Color::White);
    let black = board.pieces_by_color(Color::Black);

    let mut value = 0;

    // TODO: exclude king
    for kind in PieceKind::iter() {
        let kind_value = match kind {
            PieceKind::Pawn => 100,
            PieceKind::Knight => 300,
            PieceKind::Bishop => 300,
            PieceKind::Rook => 500,
            PieceKind::Queen => 900,
            PieceKind::King => 0,
        };
        let pieces = board.pieces_by_kind(kind);
        let balance = (pieces & white).popcount() as i32 - (pieces & black).popcount() as i32;

        value += balance * kind_value;
    }

    value
}

pub fn piece_positions(board: &Board) -> i32 {
    let mut value = 0;

    for rank in Rank::iter() {
        for file in File::iter() {
            let square = Square::new(rank, file);
            if let Some(piece) = board.get(square) {
                let v = if piece.color() == Color::White {
                    // Mirror rank
                    let rank = 7 - square.rank() as usize;
                    PIECE_SQUARE_TABLES[piece.kind() as usize][rank * 8 + square.file() as usize]
                } else {
                    -PIECE_SQUARE_TABLES[piece.kind() as usize][square as usize]
                };

                value += v as i32;
            }
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use crate::board::{piece::Piece, square::Square, Board};

    use crate::eval::piece_positions;

    #[test]
    fn piece_square() {
        let mut board = Board::new();
        board.set(Piece::WhitePawn, Square::E4);
        assert_eq!(piece_positions(&board), 20);

        board = Board::new();
        board.set(Piece::BlackPawn, Square::A2);
        assert_eq!(piece_positions(&board), -50);

        board = Board::new();
        board.set(Piece::BlackRook, Square::E2);
        assert_eq!(piece_positions(&board), -10);
    }
}
