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
    #[error("encountered a parsing error")]
    Parse(#[from] ParseBracketError),
}

#[derive(Debug, PartialEq, Error)]
enum ParseBracketError {
    #[error("expected {expected:?}, but found {found:?} instead")]
    Corrupt { expected: Bracket, found: Bracket },
    #[error("incomplete brackets: {0:?}")]
    Incomplete(Vec<Bracket>),
    #[error("invalid token: {0}")]
    Invalid(String),
}

impl ParseBracketError {
    fn score(&self) -> u64 {
        // NOTE: Score is only applicable to right brackets. It should be
        // impossible to have left brackets considered in a score, but for the
        // sake of enum exhaustiveness we just consider left brackets as having
        // no impact on the score.
        match self {
            Self::Invalid(_) => 0,
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

#[derive(Debug, Copy, Clone, PartialEq)]
enum BracketKind {
    Parens,
    Square,
    Curly,
    Angle,
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
            _ => Err(ParseBracketError::Invalid(value.to_string())),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Brackets(Vec<Bracket>);

impl IntoIterator for Brackets {
    type Item = Bracket;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::str::FromStr for Brackets {
    type Err = ParseBracketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<Bracket> = vec![];
        let mut brackets: Vec<Bracket> = vec![];

        for c in s.chars() {
            let bracket = Bracket::try_from(c)?;
            match bracket {
                Bracket::Left(_) => {
                    stack.push(bracket);
                    brackets.push(bracket);
                }
                Bracket::Right(right) => match stack.pop() {
                    // Take a left bracket off the stack.
                    Some(Bracket::Left(left)) => {
                        // Push the bracket if it matches.
                        if left == right {
                            brackets.push(bracket);
                        // Otherwise this is corruption!
                        } else {
                            return Err(ParseBracketError::Corrupt {
                                expected: Bracket::Right(left),
                                found: Bracket::Right(right),
                            });
                        }
                    }
                    // The compiler can't know only left brackets go on the stack.
                    Some(Bracket::Right(_)) => unreachable!(),
                    // The stack was empty, but we found a right bracket. Invalid.
                    None => {
                        return Err(ParseBracketError::Invalid(c.to_string()));
                    }
                },
            }
        }

        // The brackets were valid, but incomplete.
        if stack.len() > 0 {
            let closing = stack.iter().rev().map(|b| b.pair()).collect();
            Err(ParseBracketError::Incomplete(closing))
        // The brackets were valid and complete.
        } else {
            Ok(Brackets(brackets))
        }
    }
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    let reader = BufReader::new(File::open(args.input)?);
    let (part1, mut part2) = reader
        .lines()
        .try_fold::<(u64, Vec<u64>), _, Result<(u64, Vec<u64>), Error>>(
            (0, vec![]),
            |(mut part1, mut part2), line| {
                let line = line.map_err(Error::Io)?;
                let brackets: Result<Brackets, _> = line.parse();

                match brackets {
                    // We don't care about valid brackets.
                    Ok(_) => {}
                    // Corruption score counts towards part 1.
                    Err(error @ ParseBracketError::Corrupt { .. }) => {
                        part1 += error.score();
                    }
                    // Incomplete score counts towards part 2.
                    Err(error @ ParseBracketError::Incomplete(_)) => {
                        part2.push(error.score());
                    }
                    // All other errors are fatal, so we stop early.
                    Err(error) => return Err(error).map_err(Error::Parse),
                }
                Ok((part1, part2))
            },
        )?;

    println!("Part 1: {}", part1);

    // Order of equal elements doesn't matter, we don't need a stable sort.
    part2.sort_unstable();
    let mid = part2.len() / 2;

    // It's possible that there are no incomplete brackets, which would make it
    // unsafe to index the median value. In that case we'll report "N/A".
    if let Some(median) = part2.get(mid) {
        println!("Part 2: {}", median);
    } else {
        println!("Part 2: N/A");
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{Bracket, BracketKind, Brackets, ParseBracketError};

    #[test]
    fn valid_brackets() {
        let expected = Brackets(vec![
            Bracket::Left(BracketKind::Square),
            Bracket::Left(BracketKind::Angle),
            Bracket::Right(BracketKind::Angle),
            Bracket::Left(BracketKind::Parens),
            Bracket::Left(BracketKind::Curly),
            Bracket::Right(BracketKind::Curly),
            Bracket::Right(BracketKind::Parens),
            Bracket::Left(BracketKind::Curly),
            Bracket::Right(BracketKind::Curly),
            Bracket::Left(BracketKind::Square),
            Bracket::Left(BracketKind::Parens),
            Bracket::Left(BracketKind::Square),
            Bracket::Right(BracketKind::Square),
            Bracket::Right(BracketKind::Parens),
            Bracket::Left(BracketKind::Angle),
            Bracket::Right(BracketKind::Angle),
            Bracket::Right(BracketKind::Square),
            Bracket::Right(BracketKind::Square),
        ]);
        let brackets: Brackets = "[<>({}){}[([])<>]]"
            .parse()
            .expect("expected valid brackets");
        assert_eq!(brackets, expected);
    }

    #[test]
    fn corrupt_parens() {
        let error = "[[<[([]))<([[{}[[()]]]"
            .parse::<Brackets>()
            .expect_err("expected corrupt parens");
        assert_eq!(error.score(), 3);
    }

    #[test]
    fn corrupt_square_bracket() {
        let error = "[{[{({}]{}}([{[{{{}}([]"
            .parse::<Brackets>()
            .expect_err("expected corrupt square");
        assert_eq!(error.score(), 57);
    }

    #[test]
    fn corrupt_curly_brace() {
        let error = "{([(<{}[<>[]}>{[]{[(<()>"
            .parse::<Brackets>()
            .expect_err("expected corrupt curly");
        assert_eq!(error.score(), 1197);
    }

    #[test]
    fn corrupt_angle_bracket() {
        let error = "<{([([[(<>()){}]>(<<{{"
            .parse::<Brackets>()
            .expect_err("expected corrupt angle");
        assert_eq!(error.score(), 25137);
    }

    #[test]
    fn incomplete_brackets() {
        let error = "<{(["
            .parse::<Brackets>()
            .expect_err("expected incomplete brackets");
        assert_eq!(error.score(), 294);
    }

    #[test]
    fn invalid_bracket() {
        let error = "{a}"
            .parse::<Brackets>()
            .expect_err("expected invalid bracket");
        assert_eq!(error, ParseBracketError::Invalid("a".to_string()));
    }

    #[test]
    fn invalid_bracket_right_only() {
        let error = "}"
            .parse::<Brackets>()
            .expect_err("expected invalid bracket");
        assert_eq!(error, ParseBracketError::Invalid("}".to_string()));
    }
}
