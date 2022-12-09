use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Clone, Copy)]
struct Opponent(Shape);

#[derive(Debug, Clone, Copy)]
struct Player(Shape);

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

use Outcome::*;
use Shape::*;

pub struct Round {
    opponent: Opponent,
    player: Player,
}

impl Round {
    fn outcome(&self) -> Outcome {
        match (self.opponent, self.player) {
            (Opponent(Rock), Player(Rock)) => Draw,
            (Opponent(Rock), Player(Paper)) => Win,
            (Opponent(Rock), Player(Scissors)) => Loss,
            (Opponent(Paper), Player(Rock)) => Loss,
            (Opponent(Paper), Player(Paper)) => Draw,
            (Opponent(Paper), Player(Scissors)) => Win,
            (Opponent(Scissors), Player(Rock)) => Win,
            (Opponent(Scissors), Player(Paper)) => Loss,
            (Opponent(Scissors), Player(Scissors)) => Draw,
        }
    }

    pub fn score(&self) -> u64 {
        self.player.0 as u64 + self.outcome() as u64
    }
}

#[derive(Debug, Error)]
#[error("Invalid opponent: {0}")]
pub struct ParseOpponentError(String);

impl FromStr for Opponent {
    type Err = ParseOpponentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self(Rock)),
            "B" => Ok(Self(Paper)),
            "C" => Ok(Self(Scissors)),
            _ => Err(ParseOpponentError(s.to_string())),
        }
    }
}

#[derive(Debug, Error)]
#[error("Invalid player: {0}")]
pub struct ParsePlayerError(String);

impl FromStr for Player {
    type Err = ParsePlayerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self(Rock)),
            "Y" => Ok(Self(Paper)),
            "Z" => Ok(Self(Scissors)),
            _ => Err(ParsePlayerError(s.to_string())),
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseRoundError {
    #[error(transparent)]
    Opponent(#[from] ParseOpponentError),
    #[error(transparent)]
    Player(#[from] ParsePlayerError),
    #[error("Missing round participant")]
    MissingParticipant,
}

impl FromStr for Round {
    type Err = ParseRoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        match (split.next(), split.next()) {
            (Some(opponent), Some(player)) => Ok(Self {
                opponent: opponent.parse()?,
                player: player.parse()?,
            }),
            _ => Err(ParseRoundError::MissingParticipant),
        }
    }
}

pub fn total_score(rounds: impl IntoIterator<Item = Round>) -> u64 {
    rounds.into_iter().map(|r| r.score()).sum()
}

#[derive(Debug, Parser)]
struct Args {
    input: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let reader = BufReader::new(File::open(args.input)?);
    let lines = reader.lines().collect::<Result<Vec<String>, _>>()?;
    let rounds = lines
        .iter()
        .map(|r| r.parse())
        .collect::<Result<Vec<Round>, _>>()?;

    let part1 = total_score(rounds);
    println!("Part 1: {part1}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::ParseRoundError;

    use super::total_score;

    #[test]
    fn part1() -> Result<(), ParseRoundError> {
        let rounds = ["A Y".parse()?, "B X".parse()?, "C Z".parse()?];
        let score = total_score(rounds);
        assert_eq!(score, 15);
        Ok(())
    }
}
