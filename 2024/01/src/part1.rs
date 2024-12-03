#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("The line was missing a location ID")]
    MissingLocationId,
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
}

fn parse_line(line: impl AsRef<str>) -> Result<(u32, u32), Error> {
    let mut parts = line.as_ref().split_whitespace();
    let Some(left) = parts.next() else {
        return Err(Error::MissingLocationId);
    };
    let Some(right) = parts.next() else {
        return Err(Error::MissingLocationId);
    };
    let left: u32 = left.parse()?;
    let right: u32 = right.parse()?;
    Ok((left, right))
}

pub fn solve(lines: impl IntoIterator<Item = impl AsRef<str>>) -> anyhow::Result<u32> {
    let (mut left, mut right): (Vec<_>, Vec<_>) = lines
        .into_iter()
        .map(parse_line)
        .collect::<Result<Vec<(u32, u32)>, _>>()?
        .iter()
        .cloned()
        .unzip();

    left.sort();
    right.sort();

    let sum = left
        .iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use super::{parse_line, Error};

    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case("3   4", (3, 4))]
    #[test_case("4   3", (4, 3))]
    #[test_case("2   5", (2, 5))]
    #[test_case("1   3", (1, 3))]
    #[test_case("3   9", (3, 9))]
    #[test_case("3   3", (3, 3))]
    fn parsing(line: &str, expected: (u32, u32)) -> anyhow::Result<()> {
        let actual = parse_line(line)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test_case("" ; "empty")]
    #[test_case("42   " ; "missing left")]
    #[test_case("   42" ; "missing right")]
    fn missing_location_id(line: &str) {
        assert_eq!(parse_line(line), Err(Error::MissingLocationId));
    }

    #[test_case("a   42" ; "invalid left")]
    #[test_case("42   a" ; "invalid right")]
    fn invalid_location_id(line: &str) {
        assert!(matches!(
            parse_line(line),
            Err(Error::Parse(ParseIntError { .. }))
        ));
    }
}
