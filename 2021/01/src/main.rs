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
}

fn part1(numbers: &Vec<i32>) -> i32 {
    if numbers.len() <= 1 {
        return 0;
    }

    let mut total = 0;
    let mut a = numbers[0];
    for b in numbers[1..].iter() {
        if b > &a {
            total += 1;
        }
        a = *b;
    }

    total
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    let reader = BufReader::new(File::open(args.input).map_err(Error::Io)?);
    let numbers = reader
        .lines()
        .map(|line| line.map_err(Error::Io)?.parse().map_err(Error::Parse))
        .collect::<Result<Vec<i32>, _>>()?;
    println!("Part 1: {}", part1(&numbers));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::part1;

    #[test]
    fn test_part1_with_zero_numbers() {
        let numbers = vec![];
        assert_eq!(part1(&numbers), 0);
    }

    #[test]
    fn test_part1_with_one_number() {
        let numbers = vec![199];
        assert_eq!(part1(&numbers), 0);
    }

    #[test]
    fn test_part1_with_numbers() {
        let numbers = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(part1(&numbers), 7);
    }
}
