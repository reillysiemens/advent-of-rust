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

fn sonar_sweep(depths: &[u32], window: usize) -> u32 {
    if depths.len() <= window {
        return 0;
    }

    let windows = depths.windows(window).collect::<Vec<_>>();
    let increases = windows
        .iter()
        .zip(windows[1..].iter())
        .fold(0, |acc, (m1, m2)| {
            let sum1: u32 = m1.iter().sum();
            let sum2: u32 = m2.iter().sum();
            acc + if sum2 > sum1 { 1 } else { 0 }
        });

    increases
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    let reader = BufReader::new(File::open(args.input).map_err(Error::Io)?);
    let depths = reader
        .lines()
        .map(|line| line.map_err(Error::Io)?.parse().map_err(Error::Parse))
        .collect::<Result<Vec<u32>, _>>()?;

    let part1 = sonar_sweep(&depths, 1);
    let part2 = sonar_sweep(&depths, 3);
    println!("Part 1: {}\nPart 2: {}", part1, part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::sonar_sweep;

    #[test]
    fn test_sonar_sweep_with_zero_depths() {
        let depths = vec![];
        assert_eq!(sonar_sweep(&depths, 1), 0);
    }

    #[test]
    fn test_sonar_sweep_with_one_depth() {
        let depths = vec![199];
        assert_eq!(sonar_sweep(&depths, 1), 0);
    }

    #[test]
    fn test_sonar_sweep_with_depths_window_1() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(sonar_sweep(&depths, 1), 7);
    }

    #[test]
    fn test_sonar_sweep_with_depths_window_3() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(sonar_sweep(&depths, 3), 5);
    }
}
