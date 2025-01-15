use chess::board::{
    moves::{Move, MoveKind},
    piece::PieceKind,
    square::Square,
    Board,
};

#[derive(Debug)]
pub enum Command {
    Uci,
    Debug(bool),
    IsReady,
    SetOption(String, Option<String>),
    // Register,
    UciNewGame,
    Position(String, Vec<String>),
    Go(SearchParams),
    Stop,
    PonderHit,
    Quit,

    // Non-uci debug commands
    Display,
    Perft(usize),
}

#[derive(Debug)]
pub struct SearchParams {
    pub searchmoves: Vec<String>,
    pub ponder: bool,
    pub wtime: Option<usize>,
    pub btime: Option<usize>,
    pub winc: Option<usize>,
    pub binc: Option<usize>,
    pub movestogo: Option<usize>,
    pub depth: Option<usize>,
    pub nodes: Option<usize>,
    pub mate: Option<usize>,
    pub movetime: Option<usize>,
    pub infinite: bool,
}

#[derive(Debug)]
pub enum ParseCommandError {
    Invalid,
    Unknown,
    Unsupported(String),
}

impl std::fmt::Display for ParseCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseCommandError::Invalid => write!(f, "Invalid UCI command"),
            ParseCommandError::Unknown => write!(f, "Unknown UCI command"),
            ParseCommandError::Unsupported(s) => {
                write!(f, "Unsupported UCI command '{}'", s)
            }
        }
    }
}

impl std::error::Error for ParseCommandError {}

impl std::str::FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();

        match tokens.next().ok_or(ParseCommandError::Unknown)? {
            "uci" => Ok(Command::Uci),
            "isready" => Ok(Command::IsReady),
            "position" => {
                let fen = match tokens.next().ok_or(ParseCommandError::Invalid)? {
                    "startpos" => Board::STARTPOS.to_string(),
                    "fen" => tokens
                        .by_ref()
                        .take_while(|token| *token != "moves")
                        .fold(String::new(), |a, b| a + " " + b)
                        .trim()
                        .to_string(),
                    _ => return Err(ParseCommandError::Invalid),
                };

                let moves = tokens
                    .filter(|mv| *mv != "moves")
                    .map(|mv| mv.to_string())
                    .collect();
                Ok(Command::Position(fen, moves))
            }
            "go" => {
                let mut params = SearchParams {
                    searchmoves: vec![],
                    ponder: false,
                    wtime: None,
                    btime: None,
                    winc: None,
                    binc: None,
                    movestogo: None,
                    depth: None,
                    nodes: None,
                    mate: None,
                    movetime: None,
                    infinite: false,
                };

                macro_rules! parse_usize {
                    ($t: ident) => {
                        Some(
                            $t.next()
                                .ok_or(ParseCommandError::Invalid)?
                                .parse::<usize>()
                                .map_err(|_| ParseCommandError::Invalid)?,
                        )
                    };
                }

                while let Some(token) = tokens.next() {
                    // TODO: maybe implement all params?
                    match token {
                        "wtime" => params.wtime = parse_usize!(tokens),
                        "btime" => params.btime = parse_usize!(tokens),
                        "winc" => params.winc = parse_usize!(tokens),
                        "binc" => params.binc = parse_usize!(tokens),
                        "depth" => params.depth = parse_usize!(tokens),
                        "nodes" => params.nodes = parse_usize!(tokens),
                        "mate" => params.mate = parse_usize!(tokens),
                        "movetime" => params.movetime = parse_usize!(tokens),
                        "infinite" => params.infinite = true,
                        _ => return Err(ParseCommandError::Invalid),
                    }
                }

                Ok(Command::Go(params))
            }
            "stop" => Ok(Command::Stop),
            "quit" => Ok(Command::Quit),
            "d" => Ok(Command::Display),
            "perft" => {
                let depth = tokens
                    .next()
                    .ok_or(ParseCommandError::Invalid)?
                    .parse::<usize>()
                    .map_err(|_| ParseCommandError::Invalid)?;
                Ok(Command::Perft(depth))
            }
            "debug" => Err(ParseCommandError::Unsupported(String::from("debug"))),
            "setoption" => Err(ParseCommandError::Unsupported(String::from("setoption"))),
            "ucinewgame" => Err(ParseCommandError::Unsupported(String::from("ucinewgame"))),
            "ponderhit" => Err(ParseCommandError::Unsupported(String::from("ponderhit"))),
            _ => Err(ParseCommandError::Unknown),
        }
    }
}

pub fn parse_move(board: &Board, m: &str) -> Result<Move, ()> {
    if m.len() > 5 {
        return Err(());
    }
    let from: Square = m.chars().take(2).collect::<String>().parse()?;
    let to: Square = m.chars().skip(2).take(2).collect::<String>().parse()?;

    if let Some(c) = m.chars().nth(4) {
        let promotion_kind: PieceKind = c.try_into()?;
        let mv = board.moves().into_iter().find(|mv| match mv.kind() {
            MoveKind::Promotion(kind) | MoveKind::PromotionCapture(kind) => {
                mv.from() == from && mv.to() == to && kind == promotion_kind
            }
            _ => false,
        });
        mv.ok_or(())
    } else {
        let mv = board
            .moves()
            .into_iter()
            .find(|mv| mv.from() == from && mv.to() == to);

        mv.ok_or(())
    }
}
