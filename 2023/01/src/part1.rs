#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("The line was missing a digit")]
    MissingDigit,
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
}

fn calibrate(line: impl AsRef<str>) -> Result<u32, Error> {
    let mut chars = line.as_ref().chars();
    let front = chars.find(char::is_ascii_digit);
    let back = chars.rfind(char::is_ascii_digit);
    match (front, back) {
        (None, _) => Err(Error::MissingDigit),
        (Some(first), None) => Ok(format!("{first}{first}").parse()?),
        (Some(first), Some(last)) => Ok(format!("{first}{last}").parse()?),
    }
}

pub fn solve(lines: impl IntoIterator<Item = impl AsRef<str>>) -> anyhow::Result<u32> {
    Ok(lines
        .into_iter()
        .map(calibrate)
        .collect::<Result<Vec<u32>, Error>>()?
        .iter()
        .sum())
}

#[cfg(test)]
mod tests {
    use super::{calibrate, Error};
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case("1abc2", 12)]
    #[test_case("pqr3stu8vwx", 38)]
    #[test_case("a1b2c3d4e5f", 15)]
    #[test_case("treb7uchet", 77)]
    fn calibration(line: &str, expected: u32) -> anyhow::Result<()> {
        let actual = calibrate(line)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test_case("")]
    #[test_case("a")]
    #[test_case("one" ; "Only valid in part 2")]
    #[test_case("ten")]
    fn missing_digit(line: &str) {
        assert_eq!(calibrate(line), Err(Error::MissingDigit))
    }
}
