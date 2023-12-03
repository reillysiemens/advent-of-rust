use clap::Parser;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

#[derive(Debug, Parser)]
struct Args {
    input: PathBuf,
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to parse cube: '{0}'")]
struct ParseCubeError(String);

#[derive(Debug, thiserror::Error)]
#[error("Failed to parse game: '{0}'")]
struct ParseGameError(String);

#[derive(Debug, PartialEq)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl FromStr for Cube {
    type Err = ParseCubeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split(' ');
        let (Some(count), Some(color)) = (parts.next(), parts.next()) else {
            return Err(ParseCubeError(input.into()));
        };
        let count: u32 = count.parse().map_err(|_| ParseCubeError(input.into()))?;

        match color {
            "red" => Ok(Self::Red(count)),
            "green" => Ok(Self::Green(count)),
            "blue" => Ok(Self::Blue(count)),
            _ => Err(ParseCubeError(input.into())),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<Vec<Cube>>,
}

impl Game {
    fn possible(&self, red: u32, green: u32, blue: u32) -> bool {
        for set in &self.sets {
            for cube in set {
                match cube {
                    Cube::Red(count) => {
                        if *count > red {
                            return false;
                        }
                    }
                    Cube::Green(count) => {
                        if *count > green {
                            return false;
                        }
                    }
                    Cube::Blue(count) => {
                        if *count > blue {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split(": ");
        let (Some(game), Some(sets)) = (parts.next(), parts.next()) else {
            return Err(ParseGameError(input.into()));
        };

        let mut parts = game.split(' ');
        let (Some("Game"), Some(id)) = (parts.next(), parts.next()) else {
            return Err(ParseGameError(input.into()));
        };
        let id: u32 = id.parse().map_err(|_| ParseGameError(input.into()))?;

        let sets = sets
            .split("; ")
            .map(|s| {
                s.split(", ")
                    .map(|c| c.parse())
                    .collect::<Result<Vec<Cube>, _>>()
            })
            .collect::<Result<Vec<Vec<Cube>>, _>>()
            .map_err(|_| ParseGameError(input.into()))?;

        Ok(Game { id, sets })
    }
}

fn part1(
    lines: impl IntoIterator<Item = impl AsRef<str>>,
    red: u32,
    green: u32,
    blue: u32,
) -> anyhow::Result<u32> {
    let mut sum = 0;
    for line in lines.into_iter() {
        let game: Game = line.as_ref().parse()?;
        if game.possible(red, green, blue) {
            sum += game.id;
        }
    }
    Ok(sum)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let reader = BufReader::new(File::open(args.input)?);
    let lines = reader.lines().collect::<Result<Vec<String>, _>>()?;
    let part1 = part1(lines, 12, 13, 14)?;
    println!("Part 1: {part1}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Cube::*, Game};
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        Game { id: 1, sets: vec![vec![Blue(3), Red(4)], vec![Red(1), Green(2), Blue(6)], vec![Green(2)]]}
        ; "Game 1"
    )]
    #[test_case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        Game { id: 2, sets: vec![vec![Blue(1), Green(2)], vec![Green(3), Blue(4), Red(1)], vec![Green(1), Blue(1)]]}
        ; "Game 2"
    )]
    #[test_case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        Game { id: 3, sets: vec![vec![Green(8), Blue(6), Red(20)], vec![Blue(5), Red(4), Green(13)], vec![Green(5), Red(1)]]}
        ; "Game 3"
    )]
    #[test_case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        Game { id: 4, sets: vec![vec![Green(1), Red(3), Blue(6)], vec![Green(3), Red(6)], vec![Green(3), Blue(15), Red(14)]]}
        ; "Game 4"
    )]
    #[test_case(
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        Game { id: 5, sets: vec![vec![Red(6), Blue(1), Green(3)], vec![Blue(2), Red(1), Green(2)]]}
        ; "Game 5"
    )]
    fn game_parsing(record: &str, expected: Game) -> anyhow::Result<()> {
        let actual: Game = record.parse()?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test_case(
        Game { id: 1, sets: vec![vec![Blue(3), Red(4)], vec![Red(1), Green(2), Blue(6)], vec![Green(2)]]}
        ; "Game 1"
    )]
    #[test_case(
        Game { id: 2, sets: vec![vec![Blue(1), Green(2)], vec![Green(3), Blue(4), Red(1)], vec![Green(1), Blue(1)]]}
        ; "Game 2"
    )]
    #[test_case(
        Game { id: 5, sets: vec![vec![Red(6), Blue(1), Green(3)], vec![Blue(2), Red(1), Green(2)]]}
        ; "Game 5"
    )]
    fn possible(game: Game) {
        assert!(game.possible(12, 13, 14));
    }

    #[test_case(
        Game { id: 3, sets: vec![vec![Green(8), Blue(6), Red(20)], vec![Blue(5), Red(4), Green(13)], vec![Green(5), Red(1)]]}
        ; "Game 3"
    )]
    #[test_case(
        Game { id: 4, sets: vec![vec![Green(1), Red(3), Blue(6)], vec![Green(3), Red(6)], vec![Green(3), Blue(15), Red(14)]]}
        ; "Game 4"
    )]
    fn impossible(game: Game) {
        assert!(!game.possible(12, 13, 14));
    }
}
