use aho_corasick::AhoCorasick;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("The line was missing a digit")]
    MissingDigit,
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
}

// No PartialEq impl here because aho_corasick::BuildError doesn't impl it.
#[derive(Debug, thiserror::Error)]
#[error("Calibrator failed to build")]
struct CalibratorBuildError(#[from] aho_corasick::BuildError);

struct Calibrator {
    ac: AhoCorasick,
}

impl<'a> Calibrator {
    const PATTERNS: [&'a str; 18] = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    const VALUES: [&'a str; 18] = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    ];

    fn new() -> Result<Self, CalibratorBuildError> {
        Ok(Self {
            ac: AhoCorasick::new(Self::PATTERNS)?,
        })
    }

    fn calibrate(&self, line: impl AsRef<str>) -> Result<u32, Error> {
        let mut pattern_indices = self
            .ac
            .find_overlapping_iter(line.as_ref())
            .map(|m| m.pattern().as_usize());
        match (pattern_indices.next(), pattern_indices.last()) {
            (None, _) => Err(Error::MissingDigit),
            (Some(first), None) => {
                let first = Self::VALUES[first];
                Ok(format!("{first}{first}").parse()?)
            }
            (Some(first), Some(last)) => {
                let first = Self::VALUES[first];
                let last = Self::VALUES[last];
                Ok(format!("{first}{last}").parse()?)
            }
        }
    }
}

pub fn solve(lines: impl IntoIterator<Item = impl AsRef<str>>) -> anyhow::Result<u32> {
    let calibrator = Calibrator::new()?;
    Ok(lines
        .into_iter()
        .map(|line| calibrator.calibrate(line))
        .collect::<Result<Vec<u32>, Error>>()?
        .iter()
        .sum())
}

#[cfg(test)]
mod tests {
    use super::{Calibrator, Error};
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case("one", 11)]
    #[test_case("oneight", 18 ; "I'll just use regex they said")]
    #[test_case("two1nine", 29)]
    #[test_case("eightwothree", 83)]
    #[test_case("abcone2threexyz", 13)]
    #[test_case("xtwone3four", 24)]
    #[test_case("4nineeightseven2", 42)]
    #[test_case("zoneight234", 14)]
    #[test_case("7pqrstsixteen", 76)]
    fn calibration(line: &str, expected: u32) -> anyhow::Result<()> {
        let calibrator = Calibrator::new()?;
        let actual = calibrator.calibrate(line)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test_case("")]
    #[test_case("a")]
    #[test_case("ten")]
    fn missing_digit(line: &str) -> anyhow::Result<()> {
        let calibrator = Calibrator::new()?;
        let actual = calibrator.calibrate(line);
        assert!(matches!(actual, Err(Error::MissingDigit)));
        Ok(())
    }
}
