use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use structopt::StructOpt;
use thiserror::Error;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[derive(Debug, Error)]
enum Error {
    #[error("encountered an I/O error")]
    Io(#[from] std::io::Error),
    #[error("encountered a tokenization error")]
    Parse(#[from] ParseBracketError),
}

#[derive(Debug, Error)]
#[error("invalid token `{0}`")]
struct ParseBracketError(String);

// The numeric values here are scores for corrupted brackets.
#[derive(Debug, PartialEq)]
enum BracketKind {
    Parens = 3,
    Square = 57,
    Curly = 1197,
    Angle = 25137,
}

#[derive(Debug, PartialEq)]
enum Bracket {
    Left(BracketKind),
    Right(BracketKind),
}

impl TryFrom<char> for Bracket {
    type Error = ParseBracketError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Bracket::Left(BracketKind::Parens)),
            ')' => Ok(Bracket::Right(BracketKind::Parens)),
            '[' => Ok(Bracket::Left(BracketKind::Square)),
            ']' => Ok(Bracket::Right(BracketKind::Square)),
            '{' => Ok(Bracket::Left(BracketKind::Curly)),
            '}' => Ok(Bracket::Right(BracketKind::Curly)),
            '<' => Ok(Bracket::Left(BracketKind::Angle)),
            '>' => Ok(Bracket::Right(BracketKind::Angle)),
            _ => Err(ParseBracketError(value.to_string())),
        }
    }
}

// This newtype wrapper isn't necessary. I just wanted to write
// ```
// let score = "({}<>[])".parse()?.score();
// ```
// which I think looks pretty clean.
#[derive(Debug)]
struct Brackets(Vec<Bracket>);

// Boilerplate to use Brackets as an iterator.
impl IntoIterator for Brackets {
    type Item = Bracket;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// Boilerplate to allow parsing Brackets from a string.
impl std::str::FromStr for Brackets {
    type Err = ParseBracketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let brackets = s
            .chars()
            .map(Bracket::try_from)
            .collect::<Result<Vec<Bracket>, Self::Err>>()?;

        Ok(Brackets(brackets))
    }
}

impl Brackets {
    fn score(self) -> u16 {
        let mut stack: Vec<Bracket> = vec![];

        for bracket in self {
            match bracket {
                Bracket::Left(_) => stack.push(bracket),
                Bracket::Right(right) => match stack.pop() {
                    // Take a left bracket off the stack.
                    Some(Bracket::Left(left)) => {
                        // Do nothing if left and right match. Otherwise, corruption!
                        if left != right {
                            return right as u16;
                        }
                    }
                    // The compiler can't know only left brackets go on the stack.
                    Some(Bracket::Right(_)) => unreachable!(),
                    // The stack was empty, but we found a right bracket. Invalid?
                    None => {
                        return 0;
                    }
                },
            }
        }

        // This is a complete, valid set of brackets.
        0
    }
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    let reader = BufReader::new(File::open(args.input)?);
    let lines = reader
        .lines()
        .map(|line| line.map_err(Error::Io)?.parse().map_err(Error::Parse))
        .collect::<Result<Vec<Brackets>, _>>()?;

    let mut part1: u64 = 0;
    for line in lines {
        part1 += line.score() as u64;
    }

    println!("Part 1: {}", part1);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::Brackets;

    #[test]
    fn valid_brackets() {
        let brackets: Brackets = "[<>({}){}[([])<>]]".parse().unwrap();
        assert_eq!(brackets.score(), 0);
    }

    #[test]
    fn corrupt_parens() {
        let brackets: Brackets = "[[<[([]))<([[{}[[()]]]".parse().unwrap();
        assert_eq!(brackets.score(), 3);
    }

    #[test]
    fn corrupt_square_bracket() {
        let brackets: Brackets = "[{[{({}]{}}([{[{{{}}([]".parse().unwrap();
        assert_eq!(brackets.score(), 57);
    }

    #[test]
    fn corrupt_curly_brace() {
        let brackets: Brackets = "{([(<{}[<>[]}>{[]{[(<()>".parse().unwrap();
        assert_eq!(brackets.score(), 1197);
    }

    #[test]
    fn corrupt_angle_bracket() {
        let brackets: Brackets = "<{([([[(<>()){}]>(<<{{".parse().unwrap();
        assert_eq!(brackets.score(), 25137);
    }
}
