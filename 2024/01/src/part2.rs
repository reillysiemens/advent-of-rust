use std::collections::HashMap;

use crate::pair;

pub fn solve(lines: impl IntoIterator<Item = impl AsRef<str>>) -> anyhow::Result<u32> {
    let (left, right): (Vec<_>, Vec<_>) = lines
        .into_iter()
        .map(pair::parse)
        .collect::<Result<Vec<(u32, u32)>, _>>()?
        .iter()
        .cloned()
        .unzip();

    let mut map: HashMap<u32, u32> = HashMap::new();
    for value in right {
        *map.entry(value).or_default() += 1;
    }

    Ok(left.iter().map(|v| v * (*map.entry(*v).or_default())).sum())
}

#[cfg(test)]
mod tests {
    use super::solve;

    use pretty_assertions::assert_eq;

    #[test]
    fn solution() -> anyhow::Result<()> {
        let expected = 31;
        let puzzle = ["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"];
        let actual = solve(puzzle)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
