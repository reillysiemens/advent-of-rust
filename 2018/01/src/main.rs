use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, BufRead};
use std::num;

#[derive(Debug)]
enum Error {
    ArgumentError,
    EmptyInput,
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

fn first_repeat<'a>(changes: impl Iterator<Item = &'a isize> + Clone) -> Option<isize> {
    let mut frequency = 0;
    let mut frequencies = HashSet::new();
    frequencies.insert(frequency);
    for change in changes.cycle() {
        frequency += change;
        if frequencies.contains(&frequency) {
            return Some(frequency);
        }
        frequencies.insert(frequency);
    }
    None
}

fn main() -> Result<(), Error> {
    let input = env::args().nth(1).ok_or(Error::ArgumentError)?;
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>()?;
    let changes = parse(lines.iter())?;

    // The program will exit early with an error if part 2 can't be computed.
    let repeat = first_repeat(changes.iter()).ok_or(Error::EmptyInput)?;
    println!("Part 1: {}", frequency(changes.iter()));
    println!("Part 2: {}", repeat);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{first_repeat, frequency, parse};

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

    #[test]
    fn first_repeat_is_none_with_no_changes() {
        let changes = vec![];
        assert_eq!(None, first_repeat(changes.iter()));
    }

    #[test]
    fn first_repeat_handles_finite_input() {
        let inputs = vec![
            (vec![1, -1], Some(0)),
            (vec![3, 3, 4, -2, -4], Some(10)),
            (vec![-6, 3, 8, 5, -6], Some(5)),
            (vec![7, 7, -2, -7, -4], Some(14)),
        ];
        for (given, expected) in inputs {
            assert_eq!(expected, first_repeat(given.iter()));
        }
    }
}
