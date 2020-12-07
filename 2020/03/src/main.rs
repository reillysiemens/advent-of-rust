use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn trees(slope: &Vec<Vec<char>>, right: usize, down: usize) -> u32 {
    match slope.iter().nth(0) {
        Some(first) => slope
            .iter()
            .step_by(down)
            .enumerate()
            .map(|(idx, row)| match row[(idx * right) % first.len()] {
                '#' => 1,
                _ => 0,
            })
            .sum(),
        None => 0,
    }
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    let mut slope = vec![];
    let reader = BufReader::new(File::open(args.input)?);

    for line in reader.lines() {
        let line = line?;
        slope.push(line.chars().collect::<Vec<char>>())
    }

    let part1 = trees(&slope, 3, 1);
    let part2: u32 = [
        trees(&slope, 1, 1),
        part1,
        trees(&slope, 5, 1),
        trees(&slope, 7, 1),
        trees(&slope, 1, 2),
    ]
    .iter()
    .product();

    println!("Part 1: {}", part1);
    println!("Part 2: {:#?}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::trees;

    #[test]
    fn test_trees() {
        let slope = vec![
            vec!['.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.'],
            vec!['.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '#'],
            vec!['.', '#', '.', '.', '.', '#', '#', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '#', '#', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '#', '.', '#', '.', '.', '.', '.', '#'],
            vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['#', '.', '#', '#', '.', '.', '.', '#', '.', '.', '.'],
            vec!['#', '.', '.', '.', '#', '#', '.', '.', '.', '.', '#'],
            vec!['.', '#', '.', '.', '#', '.', '.', '.', '#', '.', '#'],
        ];

        assert_eq!(trees(&slope, 1, 1), 2);
        assert_eq!(trees(&slope, 3, 1), 7);
        assert_eq!(trees(&slope, 5, 1), 3);
        assert_eq!(trees(&slope, 7, 1), 4);
        assert_eq!(trees(&slope, 1, 2), 2);
    }
}
