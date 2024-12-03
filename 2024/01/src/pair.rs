#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("The line was missing a location ID")]
    MissingLocationId,
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
}

pub fn parse(line: impl AsRef<str>) -> Result<(u32, u32), Error> {
    let mut parts = line.as_ref().split_whitespace();
    let left: u32 = parts.next().ok_or(Error::MissingLocationId)?.parse()?;
    let right: u32 = parts.next().ok_or(Error::MissingLocationId)?.parse()?;
    Ok((left, right))
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use super::{parse, Error};

    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case("3   4", (3, 4))]
    #[test_case("4   3", (4, 3))]
    #[test_case("2   5", (2, 5))]
    #[test_case("1   3", (1, 3))]
    #[test_case("3   9", (3, 9))]
    #[test_case("3   3", (3, 3))]
    fn parsing(line: &str, expected: (u32, u32)) -> anyhow::Result<()> {
        let actual = parse(line)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test_case("" ; "empty")]
    #[test_case("42   " ; "missing left")]
    #[test_case("   42" ; "missing right")]
    fn missing_location_id(line: &str) {
        assert_eq!(parse(line), Err(Error::MissingLocationId));
    }

    #[test_case("a   42" ; "invalid left")]
    #[test_case("42   a" ; "invalid right")]
    fn invalid_location_id(line: &str) {
        assert!(matches!(
            parse(line),
            Err(Error::Parse(ParseIntError { .. }))
        ));
    }
}
