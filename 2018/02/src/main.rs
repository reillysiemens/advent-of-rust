use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

#[derive(Debug)]
enum Error {
    ArgumentError,
    IoError(io::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

fn char_counts(string: impl AsRef<str>) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for c in string.as_ref().chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}

fn checksum<'a>(ids: impl IntoIterator<Item = impl AsRef<str>>) -> usize {
    let (twos, threes) = ids
        .into_iter()
        .map(char_counts)
        .filter_map(|c| {
            let values: HashSet<usize> = c.values().cloned().collect();
            match (values.contains(&2), values.contains(&3)) {
                (true, true) => Some((1, 1)),
                (true, false) => Some((1, 0)),
                (false, true) => Some((0, 1)),
                (false, false) => None,
            }
        })
        .fold((0, 0), |(twos, threes), (x, y)| {
            (twos + x, threes + y)
        });
    twos * threes
}

fn main() -> Result<(), Error> {
    let input = env::args().nth(1).ok_or(Error::ArgumentError)?;
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>()?;

    println!("Part 1: {}", checksum(&lines));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{char_counts, checksum};
    use std::collections::HashMap;

    #[test]
    fn char_counts_is_empty_with_no_chars() {
        let string = "".to_string();
        let expected = HashMap::new();
        assert_eq!(expected, char_counts(&string));
    }

    #[test]
    fn char_counts_counts_single_chars() {
        let string = "a".to_string();
        let expected = vec![('a', 1)]
            .iter()
            .cloned()
            .collect::<HashMap<char, usize>>();
        assert_eq!(expected, char_counts(&string));
    }

    #[test]
    fn char_counts_counts_repeated_chars() {
        let string = "aaa".to_string();
        let expected = vec![('a', 3)]
            .iter()
            .cloned()
            .collect::<HashMap<char, usize>>();
        assert_eq!(expected, char_counts(&string));
    }

    #[test]
    fn char_counts_counts_multiple_chars() {
        let string = "abbccc".to_string();
        let expected = vec![('a', 1), ('b', 2), ('c', 3)]
            .iter()
            .cloned()
            .collect::<HashMap<char, usize>>();
        assert_eq!(expected, char_counts(&string));
    }

    #[test]
    fn checksum_is_zero_with_empty_string() {
        let strings = [""];
        assert_eq!(0, checksum(&strings));
    }

    #[test]
    fn checksum_counts_twos_and_threes_correctly() {
        let strings = [
            "abcdef", // no repeats of 2 or 3
            "bababc", // 2 a and 3 b, so it counts for both.
            "abbcde", // 2 b, but no letter appears exactly 3 times.
            "abcccd", // 3 c, but no letter appears exactly 2 times.
            "aabcdd", // 2 a and 2 d, but it only counts once.
            "abcdee", // 2 e.
            "ababab", // 3 a and 3 b, but it only counts once.
        ];
        assert_eq!(12, checksum(&strings)); // 4 (twos) * 3 (threes) = 12
    }
}
