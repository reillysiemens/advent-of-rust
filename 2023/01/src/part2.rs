use aho_corasick::AhoCorasick;

const PATTERNS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

const VALUES: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "1", "2", "3", "4", "5", "6", "7", "8", "9",
];

// No PartialEq impl here because aho_corasick::BuildError doesn't impl it.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The line was missing a digit")]
    MissingDigit,
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error(transparent)]
    Build(#[from] aho_corasick::BuildError),
}

fn calibrate(ac: &AhoCorasick, line: impl AsRef<str>) -> Result<u32, Error> {
    let mut patterns = ac
        .find_overlapping_iter(line.as_ref())
        .map(|m| m.pattern().as_usize());
    let front = patterns.next();
    let back = patterns.last();
    match (front, back) {
        (None, _) => Err(Error::MissingDigit),
        (Some(first), None) => {
            let first = VALUES[first];
            Ok(format!("{first}{first}").parse()?)
        }
        (Some(first), Some(last)) => {
            let first = VALUES[first];
            let last = VALUES[last];
            Ok(format!("{first}{last}").parse()?)
        }
    }
}

pub fn solve(lines: impl IntoIterator<Item = impl AsRef<str>>) -> Result<u32, Error> {
    let ac = AhoCorasick::new(PATTERNS)?;
    Ok(lines
        .into_iter()
        .map(|line| calibrate(&ac, line))
        .collect::<Result<Vec<u32>, Error>>()?
        .iter()
        .sum())
}

#[cfg(test)]
mod tests {
    use super::{calibrate, Error, PATTERNS};
    use aho_corasick::AhoCorasick;
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
        let ac = AhoCorasick::new(PATTERNS)?;
        let actual = calibrate(&ac, line)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test_case("")]
    #[test_case("a")]
    #[test_case("ten")]
    fn missing_digit(line: &str) -> anyhow::Result<()> {
        let ac = AhoCorasick::new(PATTERNS)?;
        let actual = calibrate(&ac, line);
        assert!(matches!(actual, Err(Error::MissingDigit)));
        Ok(())
    }
}
