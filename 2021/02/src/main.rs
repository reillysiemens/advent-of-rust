use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;
use thiserror::Error;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[derive(Error, Debug)]
enum Error {
    #[error("encountered an I/O error")]
    Io(#[from] std::io::Error),
    #[error("encountered a command parsing error")]
    Parse(#[from] ParseCommandError),
}

#[derive(Error, Debug, PartialEq)]
#[error("invalid command")]
struct ParseCommandError;

impl From<std::num::ParseIntError> for ParseCommandError {
    fn from(_error: std::num::ParseIntError) -> Self {
        ParseCommandError
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let direction = parts.next().ok_or(ParseCommandError)?;
        let units: u32 = parts.next().ok_or(ParseCommandError)?.parse()?;
        match direction {
            "forward" => Ok(Command::Forward(units)),
            "down" => Ok(Command::Down(units)),
            "up" => Ok(Command::Up(units)),
            _ => Err(ParseCommandError),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Position {
    horizontal: u32,
    depth: u32,
}

impl Position {
    fn end(&self) -> u32 {
        self.horizontal * self.depth
    }
}

fn dive(commands: &[Command]) -> Position {
    let mut position = Position {
        horizontal: 0,
        depth: 0,
    };

    for command in commands {
        match command {
            Command::Forward(x) => position.horizontal += x,
            Command::Down(x) => position.depth += x,
            Command::Up(x) => position.depth -= x,
        }
    }

    position
}

fn dive_with_aim(commands: &[Command]) -> Position {
    let mut aim = 0;
    let mut position = Position {
        horizontal: 0,
        depth: 0,
    };

    for command in commands {
        match command {
            Command::Forward(x) => {
                position.horizontal += x;
                position.depth += aim * x;
            }
            Command::Down(x) => aim += x,
            Command::Up(x) => aim -= x,
        }
    }

    position
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    let reader = BufReader::new(File::open(args.input).map_err(Error::Io)?);
    let commands = reader
        .lines()
        .map(|line| line.map_err(Error::Io)?.parse().map_err(Error::Parse))
        .collect::<Result<Vec<Command>, _>>()?;

    let part1 = dive(&commands);
    let part2 = dive_with_aim(&commands);
    println!("Part 1: {}\nPart 2: {}", part1.end(), part2.end());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{dive, dive_with_aim, Command, ParseCommandError, Position};

    #[test]
    fn test_forward_command_from_string() {
        let given: Result<Command, ParseCommandError> = "forward 5".parse();
        assert_eq!(given, Ok(Command::Forward(5)));
    }

    #[test]
    fn test_down_command_from_string() {
        let given: Result<Command, ParseCommandError> = "down 5".parse();
        assert_eq!(given, Ok(Command::Down(5)));
    }

    #[test]
    fn test_up_command_from_string() {
        let given: Result<Command, ParseCommandError> = "up 3".parse();
        assert_eq!(given, Ok(Command::Up(3)));
    }

    #[test]
    fn test_invalid_command_missing_command() {
        let given: Result<Command, ParseCommandError> = "".parse();
        assert_eq!(given, Err(ParseCommandError));
    }

    #[test]
    fn test_invalid_command_missing_unit() {
        let given: Result<Command, ParseCommandError> = "forward".parse();
        assert_eq!(given, Err(ParseCommandError));
    }

    #[test]
    fn test_invalid_command_invalid_unit() {
        let given: Result<Command, ParseCommandError> = "forward a".parse();
        assert_eq!(given, Err(ParseCommandError));
    }

    #[test]
    fn test_position_end() {
        let position = Position {
            horizontal: 15,
            depth: 10,
        };
        assert_eq!(position.end(), 150);
    }

    #[test]
    fn test_dive() {
        let given = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];
        let expected = Position {
            horizontal: 15,
            depth: 10,
        };
        assert_eq!(dive(&given), expected);
    }

    #[test]
    fn test_dive_with_aim() {
        let given = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];
        let expected = Position {
            horizontal: 15,
            depth: 60,
        };
        assert_eq!(dive_with_aim(&given), expected);
    }
}
