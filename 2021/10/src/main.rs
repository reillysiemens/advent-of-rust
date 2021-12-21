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
#[error("invalid token: {0}")]
struct ParseBracketError(String);

#[derive(Debug, Error)]
enum BracketEvalError {
    #[error("expected {expected:?}, but found {found:?} instead")]
    Corrupt { expected: Bracket, found: Bracket },
    #[error("incomplete brackets: {0:?}")]
    Incomplete(Vec<Bracket>),
}

impl BracketEvalError {
    fn score(&self) -> u64 {
        // NOTE: Score is only applicable to right brackets. It should be
        // impossible to have left brackets considered in a score, but for the
        // sake of enum exhaustiveness we just consider left brackets as having
        // no impact on the score.
        match self {
            Self::Corrupt { expected: _, found } => match found {
                Bracket::Right(kind) => match kind {
                    BracketKind::Parens => 3,
                    BracketKind::Square => 57,
                    BracketKind::Curly => 1197,
                    BracketKind::Angle => 25137,
                },
                _ => 0,
            },
            Self::Incomplete(brackets) => brackets.iter().fold(0, |acc, b| match b {
                Bracket::Right(kind) => {
                    (match kind {
                        BracketKind::Parens => 1,
                        BracketKind::Square => 2,
                        BracketKind::Curly => 3,
                        BracketKind::Angle => 4,
                    }) + (acc * 5)
                }
                _ => acc,
            }),
        }
    }
}

#[derive(Debug, PartialEq)]
enum BracketKind {
    Parens,
    Square,
    Curly,
    Angle,
}

#[derive(Debug, PartialEq)]
enum Bracket {
    Left(BracketKind),
    Right(BracketKind),
}

impl Bracket {
    fn pair(&self) -> Self {
        match self {
            Bracket::Left(BracketKind::Parens) => Bracket::Right(BracketKind::Parens),
            Bracket::Right(BracketKind::Parens) => Bracket::Left(BracketKind::Parens),
            Bracket::Left(BracketKind::Square) => Bracket::Right(BracketKind::Square),
            Bracket::Right(BracketKind::Square) => Bracket::Left(BracketKind::Square),
            Bracket::Left(BracketKind::Curly) => Bracket::Right(BracketKind::Curly),
            Bracket::Right(BracketKind::Curly) => Bracket::Left(BracketKind::Curly),
            Bracket::Left(BracketKind::Angle) => Bracket::Right(BracketKind::Angle),
            Bracket::Right(BracketKind::Angle) => Bracket::Left(BracketKind::Angle),
        }
    }
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
    fn eval(self) -> Result<(), BracketEvalError> {
        let mut stack: Vec<Bracket> = vec![];

        for bracket in self {
            match bracket {
                Bracket::Left(_) => stack.push(bracket),
                Bracket::Right(right) => match stack.pop() {
                    // Take a left bracket off the stack.
                    Some(Bracket::Left(left)) => {
                        // If the left and right don't match we found
                        // corruption, otherwise do nothing.
                        if left != right {
                            return Err(BracketEvalError::Corrupt {
                                expected: Bracket::Right(left),
                                found: Bracket::Right(right),
                            });
                        }
                    }
                    // The compiler can't know only left brackets go on the stack.
                    Some(Bracket::Right(_)) => unreachable!(),
                    // The stack was empty, but we found a right bracket. Invalid?
                    None => {
                        todo!("WTF do we do in this case?")
                    }
                },
            }
        }

        // The brackets were valid, but incomplete.
        if stack.len() > 0 {
            let closing = stack.iter().rev().map(|b| b.pair()).collect();
            Err(BracketEvalError::Incomplete(closing))
        // The brackets were valid and complete.
        } else {
            Ok(())
        }
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
    let mut part2: Vec<u64> = vec![];

    for brackets in lines {
        match brackets.eval() {
            Ok(_) => {}
            Err(error @ BracketEvalError::Corrupt { .. }) => {
                part1 += error.score();
            }
            Err(error @ BracketEvalError::Incomplete(_)) => {
                part2.push(error.score());
            }
        }
    }

    // We don't need a stable sort, order of equal elements doesn't matter for this.
    part2.sort_unstable();
    let mid = part2.len() / 2;
    let part2 = part2[mid];

    println!("Part 1: {}\nPart 2: {}", part1, part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::Brackets;

    #[test]
    fn corrupt_parens() {
        let brackets: Brackets = "[[<[([]))<([[{}[[()]]]".parse().unwrap();
        let error = brackets.eval().expect_err("expected corrupt parens");
        assert_eq!(error.score(), 3);
    }

    #[test]
    fn corrupt_square_bracket() {
        let brackets: Brackets = "[{[{({}]{}}([{[{{{}}([]".parse().unwrap();
        let error = brackets.eval().expect_err("expected corrupt square");
        assert_eq!(error.score(), 57);
    }

    #[test]
    fn corrupt_curly_brace() {
        let brackets: Brackets = "{([(<{}[<>[]}>{[]{[(<()>".parse().unwrap();
        let error = brackets.eval().expect_err("expected corrupt curly");
        assert_eq!(error.score(), 1197);
    }

    #[test]
    fn corrupt_angle_bracket() {
        let brackets: Brackets = "<{([([[(<>()){}]>(<<{{".parse().unwrap();
        let error = brackets.eval().expect_err("expected corrupt angle");
        assert_eq!(error.score(), 25137);
    }

    #[test]
    fn incomplete_brackets() {
        let brackets: Brackets = "<{([".parse().unwrap();
        let error = brackets.eval().expect_err("expected incomplete brackets");
        assert_eq!(error.score(), 294);
    }
}
