use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
    path::PathBuf,
};

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    input: PathBuf,
    #[clap(long, default_value_t = 3)]
    num_elves: usize,
}

fn push_elf(elves: &mut BinaryHeap<Reverse<u64>>, calories: &mut Vec<u64>, top: usize) {
    elves.push(Reverse(calories.iter().sum()));
    if elves.len() > top {
        elves.pop();
    }
    calories.clear();
}

fn top_calories(
    lines: impl IntoIterator<Item = impl AsRef<str>>,
    top: usize,
) -> Result<Vec<u64>, ParseIntError> {
    // Use a min heap to track elf calories. By popping all of the smallest
    // values we've seen so far we can retain only the `top` largest values. We
    // use a capacity of `top + 1` to allow for one comparison before popping.
    let mut elves = BinaryHeap::with_capacity(top + 1);
    let mut calories: Vec<u64> = vec![];

    for line in lines {
        let line = line.as_ref();
        if line.is_empty() {
            // The line is empty, so we must have just finished gathering all
            // the calories for one elf. Push their calories into the heap.
            push_elf(&mut elves, &mut calories, top);
        } else {
            // Gather up calories for the current elf.
            let item = line.parse()?;
            calories.push(item);
        }
    }

    // Push the final elf into the heap as their calories won't be separated by
    // an empty line.
    push_elf(&mut elves, &mut calories, top);

    Ok(elves.into_sorted_vec().into_iter().map(|r| r.0).collect())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let reader = BufReader::new(File::open(args.input)?);
    let lines = reader.lines().collect::<Result<Vec<String>, _>>()?;

    let calories = top_calories(lines, args.num_elves)?;
    let part1 = calories[0];
    let part2: u64 = calories.iter().sum();

    println!("Part 1: {part1}\nPart 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::top_calories;
    use std::num::ParseIntError;

    const ELVEN_CALORIES: [&str; 14] = [
        "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
        "10000",
    ];

    #[test]
    fn test_part1() -> Result<(), ParseIntError> {
        let expected = 24_000;
        let answer = top_calories(ELVEN_CALORIES, 1)?;

        assert_eq!(answer[0], expected);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), ParseIntError> {
        let answer: u64 = top_calories(ELVEN_CALORIES, 3)?.iter().sum();
        let expected = 45_000;

        assert_eq!(answer, expected);
        Ok(())
    }
}
