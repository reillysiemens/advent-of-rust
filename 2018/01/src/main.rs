use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, BufRead};
use std::num;

#[derive(Debug)]
enum Error {
    ArgumentError,
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(error: num::ParseIntError) -> Self {
        Error::ParseError(error)
    }
}

fn parse<'a>(
    changes: impl Iterator<Item = &'a (impl AsRef<str> + 'a)>,
) -> Result<Vec<isize>, num::ParseIntError> {
    changes
        .map(|c| c.as_ref().parse())
        .collect::<Result<Vec<isize>, num::ParseIntError>>()
}

fn frequency<'a>(changes: impl Iterator<Item = &'a isize>) -> isize {
    changes.sum()
}

fn main() -> Result<(), Error> {
    let input = env::args().nth(1).ok_or(Error::ArgumentError)?;
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>()?;
    let changes = parse(lines.iter())?;

    println!("Part 1: {}", frequency(changes.iter()));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{frequency, parse};

    #[test]
    fn parse_errors_with_invalid_digits() {
        let invalid_digits = vec!["", "a", "$", "‽"];
        for invalid_digit in invalid_digits {
            assert!(parse(vec![invalid_digit].iter()).is_err());
        }
    }

    #[test]
    fn parse_handles_positive_digits() {
        let digits = vec!["+1", "+1", "+1"];
        let expected = Ok(vec![1, 1, 1]);
        assert_eq!(expected, parse(digits.iter()));
    }

    #[test]
    fn parse_handles_negative_digits() {
        let digits = vec!["-1", "-2", "-3"];
        let expected = Ok(vec![-1, -2, -3]);
        assert_eq!(expected, parse(digits.iter()));
    }

    #[test]
    fn parse_handles_positive_and_negative_digits() {
        let digits = vec!["+1", "+1", "-2"];
        let expected = Ok(vec![1, 1, -2]);
        assert_eq!(expected, parse(digits.iter()));
    }

    #[test]
    fn frequency_is_zero_with_no_changes() {
        let changes: Vec<isize> = vec![];
        assert_eq!(0, frequency(changes.iter()));
    }

    #[test]
    fn frequency_handles_positive_changes() {
        let changes = vec![1, 1, 1];
        assert_eq!(3, frequency(changes.iter()));
    }

    #[test]
    fn frequency_handles_negative_changes() {
        let changes = vec![-1, -2, -3];
        assert_eq!(-6, frequency(changes.iter()));
    }

    #[test]
    fn frequency_handles_positive_and_negative_changes() {
        let changes = vec![1, 1, -2];
        assert_eq!(0, frequency(changes.iter()));
    }
}
