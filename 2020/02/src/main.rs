use std::convert::TryFrom;
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

#[derive(PartialEq, Debug)]
struct PasswordEntry<'a> {
    low: usize,
    high: usize,
    letter: char,
    password: &'a str,
}

impl<'a> PasswordEntry<'a> {
    /// Whether the password contains a number of matches for `letter` within
    /// the `low..high` range.
    fn valid_range(&self) -> bool {
        (self.low..=self.high).contains(&self.password.matches(self.letter).count())
    }

    /// Whether the password has matches for `letter` at either the `low` or
    /// `high` index, but not both!
    fn valid_indices(&self) -> bool {
        let (low, high) = self
            .password
            .chars()
            .enumerate()
            .map(|(index, letter)| (index + 1, letter))
            .fold((false, false), |(mut low, mut high), (index, letter)| {
                match (index, letter == self.letter) {
                    (idx, true) if idx == self.low => low = true,
                    (idx, true) if idx == self.high => high = true,
                    _ => {}
                }
                (low, high)
            });
        low ^ high
    }
}

#[derive(Error, Debug)]
#[error("invalid password entry")]
struct PasswordEntryError;

impl From<std::num::ParseIntError> for PasswordEntryError {
    fn from(_error: std::num::ParseIntError) -> Self {
        PasswordEntryError
    }
}

impl From<std::char::ParseCharError> for PasswordEntryError {
    fn from(_error: std::char::ParseCharError) -> Self {
        PasswordEntryError
    }
}

impl<'a> TryFrom<&'a str> for PasswordEntry<'a> {
    type Error = PasswordEntryError;

    fn try_from(string: &'a str) -> Result<Self, Self::Error> {
        let mut parts = string.split(' ');
        let mut range = parts.next().ok_or(PasswordEntryError)?.split('-');

        let low: usize = range.next().ok_or(PasswordEntryError)?.parse()?;
        let high: usize = range.next().ok_or(PasswordEntryError)?.parse()?;
        let letter: char = parts
            .next()
            .ok_or(PasswordEntryError)?
            .trim_end_matches(':')
            .parse()?;
        let password: &str = parts.next().ok_or(PasswordEntryError)?;

        Ok(Self {
            low,
            high,
            letter,
            password,
        })
    }
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    let mut part1 = 0;
    let mut part2 = 0;

    let reader = BufReader::new(File::open(args.input)?);

    for line in reader.lines() {
        let line = line?;
        let entry = PasswordEntry::try_from(line.as_ref())?;
        if entry.valid_range() {
            part1 += 1;
        }
        if entry.valid_indices() {
            part2 += 1;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::PasswordEntry;
    use std::convert::TryFrom;

    #[test]
    fn test_password_entry_from_str() {
        let given = vec![
            (
                "1-3 a: abcde",
                PasswordEntry {
                    low: 1,
                    high: 3,
                    letter: 'a',
                    password: "abcde",
                },
            ),
            (
                "1-3 b: cdefg",
                PasswordEntry {
                    low: 1,
                    high: 3,
                    letter: 'b',
                    password: "cdefg",
                },
            ),
            (
                "2-9 c: ccccccccc",
                PasswordEntry {
                    low: 2,
                    high: 9,
                    letter: 'c',
                    password: "ccccccccc",
                },
            ),
        ];

        for (string, entry) in given {
            assert_eq!(PasswordEntry::try_from(string).unwrap(), entry);
        }
    }

    #[test]
    fn test_valid_range() {
        let given = vec![
            (
                PasswordEntry {
                    low: 1,
                    high: 3,
                    letter: 'a',
                    password: "abcde",
                },
                true,
            ),
            (
                PasswordEntry {
                    low: 1,
                    high: 3,
                    letter: 'b',
                    password: "cdefg",
                },
                false,
            ),
            (
                PasswordEntry {
                    low: 2,
                    high: 9,
                    letter: 'c',
                    password: "ccccccccc",
                },
                true,
            ),
        ];

        for (entry, expected) in given {
            assert_eq!(entry.valid_range(), expected);
        }
    }

    #[test]
    fn test_valid_indices() {
        let given = vec![
            (
                PasswordEntry {
                    low: 1,
                    high: 3,
                    letter: 'a',
                    password: "abcde",
                },
                true,
            ),
            (
                PasswordEntry {
                    low: 1,
                    high: 3,
                    letter: 'b',
                    password: "cdefg",
                },
                false,
            ),
            (
                PasswordEntry {
                    low: 2,
                    high: 9,
                    letter: 'c',
                    password: "ccccccccc",
                },
                false,
            ),
        ];

        for (entry, expected) in given {
            assert_eq!(entry.valid_indices(), expected);
        }
    }
}
