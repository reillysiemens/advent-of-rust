use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
#[error("invalid bracket: {0}")]
pub struct InvalidBracket(String);

#[derive(Debug, PartialEq, Error)]
pub enum ParseBracketError {
    #[error("expected {expected:?}, but found {found:?} instead")]
    Corrupt { expected: Bracket, found: Bracket },
    #[error("incomplete brackets: {0:?}")]
    Incomplete(Vec<Bracket>),
    #[error("invalid bracket: {0}")]
    Invalid(String),
}

// XXX: This From impl is written manually to prevent Bracket::try_from
// returning impossible variants like Corrupt or Incomplete. We just grab the
// invalid character and pass it up the stack. There's probably a better way to
// do this, but I don't know what it is...
impl From<InvalidBracket> for ParseBracketError {
    fn from(error: InvalidBracket) -> ParseBracketError {
        ParseBracketError::Invalid(error.0)
    }
}

// NOTE: Scores are implemented on errors and not brackets themselves because
// they're only applicable to right brackets in the presence of corruption or
// incompleteness. Other errors or errors on left brackets (which should be
// impossible) return 0 to avoid contributing to a score.
impl ParseBracketError {
    pub fn score(&self) -> u64 {
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
            Self::Invalid(_) => 0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BracketKind {
    Parens,
    Square,
    Curly,
    Angle,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Bracket {
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
    type Error = InvalidBracket;

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
            _ => Err(InvalidBracket(value.to_string())),
        }
    }
}

// Create a small newtype wrapper around Vec<Brackets> so that we can get
// around the orphan rule and implement FromStr.
#[derive(Debug, PartialEq)]
pub struct Brackets(Vec<Bracket>);

// This boilerplate is a convenience for the newtype wrapper to allow iteration.
// We don't use it here, but it could be handy if this library evolved.
impl IntoIterator for Brackets {
    type Item = Bracket;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// This is where all the real work happens. With this we can .parse()? to get
// an Iterator of Bracket. If we don't have valid brackets then we'll get a
// descriptive error.
impl FromStr for Brackets {
    type Err = ParseBracketError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<Bracket> = vec![];
        let mut brackets: Vec<Bracket> = vec![];

        for chr in string.chars() {
            let bracket = Bracket::try_from(chr)?;
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
                        return Err(ParseBracketError::Invalid(chr.to_string()));
                    }
                },
            }
        }

        // Brackets remain on the stack, therefore they're incomplete.
        if stack.len() > 0 {
            let closing = stack.iter().rev().map(|b| b.pair()).collect();
            return Err(ParseBracketError::Incomplete(closing));
        }

        // The brackets were valid and complete.
        Ok(Brackets(brackets))
    }
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
