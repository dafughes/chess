use std::{fmt, str::FromStr};

use crate::{
    board::Board,
    eval::Score,
    moves::{generate_moves, Move},
    piece::PieceKind,
    square::{File, Rank, Square},
};

#[derive(Debug)]
pub struct SearchParams {
    pub wtime: u32,
    pub btime: u32,
    pub winc: u16,
    pub binc: u16,
    pub movestogo: Option<u16>,
    pub depth: Option<u8>,
    pub nodes: Option<u64>,
    pub mate: Option<u8>,
    pub movetime: Option<u32>,
    pub infinite: bool,
}

impl SearchParams {
    pub fn new() -> Self {
        Self {
            wtime: 0,
            btime: 0,
            winc: 0,
            binc: 0,
            movestogo: None,
            depth: None,
            nodes: None,
            mate: None,
            movetime: None,
            infinite: false,
        }
    }
}

#[derive(Debug)]
pub struct Info {
    pub depth: Option<u32>,
    // pub seldepth: Option<u32>,
    pub time: Option<u32>,
    pub nodes: Option<u64>,
    // pub pv: Vector<Move>,
    // pub multipv: u32,
    pub score: Option<Score>, // TODO: change Score to enum
    pub currmove: Option<Move>,
    pub currmovenumber: Option<u32>,
    // pub hashfull: Option<u32>,
    pub nps: Option<u64>,
    // pub tbhits: Option<u64>,
    // pub sbhits: Option<u64>,
    // pub cpuload: Option<u32>,
    // pub string: Option<String>,
    // pub refutation: Vector<Move>,
    // pub currline: Vector<Move>,
}

#[derive(Debug)]
pub enum UCICommand {
    // To engine
    Uci,
    IsReady,
    Quit,
    Stop,
    Position(String, Vec<String>),
    Go(SearchParams),

    // From engine
    Info(Info),
    Id(String, String),
    UciOk,
    ReadyOk,
    BestMove(Move),

    // Debug commands (not really UCI)
    Perft(usize),
    Display,
}

pub struct ParseUCICommandError {
    msg: String,
    line: String,
}

impl ParseUCICommandError {
    pub fn new<S: AsRef<str>>(msg: S, line: S) -> Self {
        Self {
            msg: msg.as_ref().to_owned(),
            line: line.as_ref().to_owned(),
        }
    }
}

impl fmt::Display for ParseUCICommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UCI parse error: {}, command was '{}'",
            self.msg, self.line
        )
    }
}

fn parse_position<'a, I>(mut tokens: I, line: &str) -> Result<UCICommand, ParseUCICommandError>
where
    I: Iterator<Item = &'a str>,
{
    let fen = match tokens.next().ok_or(ParseUCICommandError::new(
        "Missing position parameter",
        line,
    ))? {
        "startpos" => "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_owned(),
        "fen" => tokens
            .take_while(|&token| token != "moves")
            .collect::<Vec<_>>()
            .join(" "),
        _ => return Err(ParseUCICommandError::new("Invalid position value", line)),
    };

    // Moves
    let moves = line
        .split_whitespace()
        .skip_while(|&t| t != "moves")
        .skip(1)
        .map(|t| t.to_owned())
        .collect();

    println!("{}", fen);

    Ok(UCICommand::Position(fen, moves))
}

fn parse<'a, I, T>(tokens: &mut I, line: &str) -> Result<T, ParseUCICommandError>
where
    I: Iterator<Item = &'a str>,
    T: FromStr,
{
    tokens
        .next()
        .ok_or(ParseUCICommandError::new(
            format!("Missing {} value", std::any::type_name::<T>()).as_str(),
            line,
        ))?
        .parse::<T>()
        .map_err(|_| {
            ParseUCICommandError::new(
                format!("Could not parse {}", std::any::type_name::<T>()).as_str(),
                line,
            )
        })
}

fn parse_go<'a, I>(tokens: I, line: &str) -> Result<UCICommand, ParseUCICommandError>
where
    I: Iterator<Item = &'a str>,
{
    let mut tokens = tokens.peekable();

    match tokens
        .peek()
        .ok_or(ParseUCICommandError::new("Go parameter is empty", line))?
    {
        &"perft" => {
            tokens.next();
            Ok(UCICommand::Perft(parse(&mut tokens, line)?))
        }
        _ => {
            let mut params = SearchParams::new();

            while let Some(token) = tokens.next() {
                match token {
                    "wtime" => params.wtime = parse(&mut tokens, line)?,
                    "btime" => params.btime = parse(&mut tokens, line)?,
                    "winc" => params.winc = parse(&mut tokens, line)?,
                    "binc" => params.binc = parse(&mut tokens, line)?,
                    "movestogo" => params.movestogo = Some(parse(&mut tokens, line)?),
                    "depth" => params.depth = Some(parse(&mut tokens, line)?),
                    "nodes" => params.nodes = Some(parse(&mut tokens, line)?),
                    "mate" => params.mate = Some(parse(&mut tokens, line)?),
                    "movetime" => params.movetime = Some(parse(&mut tokens, line)?),
                    "infinite" => params.infinite = true,
                    _ => return Err(ParseUCICommandError::new("Unknown command", line)),
                }
            }

            Ok(UCICommand::Go(params))
        }
    }
}

pub fn parse_command(line: &str) -> Result<UCICommand, ParseUCICommandError> {
    let mut tokens = line.split_whitespace();

    match tokens
        .next()
        .ok_or(ParseUCICommandError::new("Command is empty", line))?
    {
        "uci" => Ok(UCICommand::Uci),
        "isready" => Ok(UCICommand::IsReady),
        "quit" => Ok(UCICommand::Quit),
        "stop" => Ok(UCICommand::Stop),
        "position" => parse_position(tokens, line),
        "go" => parse_go(tokens, line),
        "d" => Ok(UCICommand::Display),
        _ => Err(ParseUCICommandError::new("Unknown command", line)),
    }
}

pub fn parse_move(mv: &str, board: &Board) -> Result<Move, ()> {
    let mut chars = mv.chars();

    let from_file: File = chars.next().ok_or(())?.try_into()?;
    let from_rank: Rank = chars.next().ok_or(())?.try_into()?;
    let to_file: File = chars.next().ok_or(())?.try_into()?;
    let to_rank: Rank = chars.next().ok_or(())?.try_into()?;

    let from = Square::new(from_rank, from_file);
    let to = Square::new(to_rank, to_file);

    let promotion_kind: Option<PieceKind> = chars.next().and_then(|c| c.try_into().ok());

    generate_moves(board)
        .into_iter()
        .find(|m| m.from() == from && m.to() == to && m.promotion_kind() == promotion_kind)
        .ok_or(())
}
