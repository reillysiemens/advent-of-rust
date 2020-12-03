use std::collections::HashSet;
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

#[derive(Error, Debug)]
enum Error {
    #[error("encountered an I/O error")]
    Io(#[from] std::io::Error),
    #[error("encountered a parsing error")]
    Parse(#[from] std::num::ParseIntError),
    #[error("no solution was found")]
    NotFound,
}

/// Find the product of the first two numbers in the input set which sum to
/// `2020`, if they exist.
fn part1(numbers: &HashSet<i32>) -> Option<i32> {
    for number in numbers {
        let target = 2020 - number;
        if numbers.contains(&target) {
            return Some(number * target);
        }
    }
    None
}

/// Find the product of the first three numbers in the input set which sum to
/// `2020`, if they exist.
fn part2(numbers: &HashSet<i32>) -> Option<i32> {
    for outer in numbers {
        for inner in numbers {
            let target = 2020 - outer - inner;
            if numbers.contains(&target) {
                return Some(inner * outer * target);
            }
        }
    }
    None
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    let reader = BufReader::new(File::open(args.input).map_err(Error::Io)?);
    let numbers = reader
        .lines()
        .map(|line| line.map_err(Error::Io)?.parse().map_err(Error::Parse))
        .collect::<Result<HashSet<i32>, _>>()?;

    println!("Part 1: {}", part1(&numbers).ok_or(Error::NotFound)?);
    println!("Part 2: {}", part2(&numbers).ok_or(Error::NotFound)?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn test_part1_solution() {
        // 1721 + 299 = 2020, so we expect 1721 * 299.
        let numbers = [1721, 979, 366, 299, 675, 1456].iter().cloned().collect();
        assert_eq!(part1(&numbers), Some(514_579));
    }

    #[test]
    fn test_part1_no_solution() {
        // 299 has been removed from this set, so there is no solution.
        let numbers = [1721, 979, 366, 675, 1456].iter().cloned().collect();
        assert_eq!(part1(&numbers), None);
    }

    #[test]
    fn test_part2_solution() {
        // 979 + 366 + 675 = 2020, so we expect 979 * 366 * 675.
        let numbers = [1721, 979, 366, 299, 675, 1456].iter().cloned().collect();
        assert_eq!(part2(&numbers), Some(241_861_950));
    }

    #[test]
    fn test_part2_no_solution() {
        // 675 has been removed from this set, so there is no solution.
        let numbers = [1721, 979, 366, 299, 1456].iter().cloned().collect();
        assert_eq!(part2(&numbers), None);
    }
}
