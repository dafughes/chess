use crate::game::{
    color::Color,
    piece::PieceKind,
    square::{File, Rank, Square},
    Game,
};

#[derive(Debug, Clone, Copy, Default)]
pub enum Score {
    #[default]
    Draw,
    Evaluation(i32),
    Mate(i32),
    Min,
    Max,
}

impl Score {
    pub fn mate(plies: usize) -> Self {
        Score::Mate((plies as i32 + 1) / 2)
    }

    fn to_i32(self) -> i32 {
        match self {
            Score::Draw => 0,
            Score::Evaluation(score) => score,
            Score::Mate(turns) => turns.signum() * 1_000_000 - turns,
            Score::Min => -10_000_000,
            Score::Max => 10_000_000,
        }
    }
}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.to_i32() == other.to_i32()
    }
}

impl Eq for Score {}

impl std::ops::Neg for Score {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Score::Draw => Score::Draw,
            Score::Evaluation(score) => Score::Evaluation(-score),
            Score::Mate(turns) => Score::Mate(-turns),
            Score::Min => Score::Max,
            Score::Max => Score::Min,
        }
    }
}

impl std::ops::Add for Score {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Score::Evaluation(self.to_i32() + rhs.to_i32())
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_i32().cmp(&other.to_i32())
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Score::Mate(turns) => write!(f, "mate {turns}"),
            score => write!(f, "cp {}", score.to_i32()),
        }
    }
}

pub fn material(game: &Game) -> Score {
    let values = [
        (PieceKind::Pawn, 100),
        (PieceKind::Knight, 300),
        (PieceKind::Bishop, 300),
        (PieceKind::Rook, 500),
        (PieceKind::Queen, 900),
        (PieceKind::King, 0),
    ];

    let score = values.iter().fold(0, |acc, (kind, value)| {
        let pieces = game.pieces_by_kind(*kind);
        let white = game.pieces_by_color(Color::White) & pieces;
        let black = game.pieces_by_color(Color::Black) & pieces;

        acc + (white.popcount() as i32 - black.popcount() as i32) * value
    });

    Score::Evaluation(score)
}

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

pub fn piece_positions(game: &Game) -> Score {
    let mut score = 0;

    for rank in Rank::iter() {
        for file in File::iter() {
            let square = Square::new(rank, file);
            if let Some(piece) = game.get_piece(square) {
                let v = if piece.color() == Color::White {
                    // Mirror rank
                    let rank = 7 - square.rank() as usize;
                    PIECE_SQUARE_TABLES[piece.kind() as usize][rank * 8 + square.file() as usize]
                } else {
                    -PIECE_SQUARE_TABLES[piece.kind() as usize][square as usize]
                };

                score += v as i32;
            }
        }
    }

    Score::Evaluation(score)
}
