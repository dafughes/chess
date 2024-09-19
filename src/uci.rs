use std::{fmt, str::FromStr};

use crate::search::SearchParams;

#[derive(Debug)]
pub enum UCICommand {
    // To engine
    Uci,
    IsReady,
    Quit,
    Position(String, Vec<String>),
    Go(SearchParams),

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
            .collect::<String>(),
        _ => return Err(ParseUCICommandError::new("Invalid position value", line)),
    };

    // Moves
    let moves = line
        .split_whitespace()
        .skip_while(|&t| t != "moves")
        .skip(1)
        .map(|t| t.to_owned())
        .collect();

    Ok(UCICommand::Position(fen, moves))
}

fn parse<'a, I, T>(tokens: &mut I, line: &str) -> Result<T, ParseUCICommandError>
where
    I: Iterator<Item = &'a str>,
    T: FromStr,
{
    tokens
        .next()
        .ok_or(ParseUCICommandError::new("Missing int value", line))?
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
                    "movestogo" => params.movestogo = parse(&mut tokens, line)?,
                    "depth" => params.depth = parse(&mut tokens, line)?,
                    "nodes" => params.nodes = parse(&mut tokens, line)?,
                    "mate" => params.mate = parse(&mut tokens, line)?,
                    "movetime" => params.movetime = parse(&mut tokens, line)?,
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
        "position" => parse_position(tokens, line),
        "go" => parse_go(tokens, line),
        "d" => Ok(UCICommand::Display),
        _ => Err(ParseUCICommandError::new("Unknown command", line)),
    }
}
