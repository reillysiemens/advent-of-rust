use crate::pair;

pub fn solve(lines: impl IntoIterator<Item = impl AsRef<str>>) -> anyhow::Result<u32> {
    let (mut left, mut right): (Vec<_>, Vec<_>) = lines
        .into_iter()
        .map(pair::parse)
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
    use super::solve;

    #[test]
    fn solution() -> anyhow::Result<()> {
        let expected = 11;
        let puzzle = ["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"];
        let actual = solve(puzzle)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
