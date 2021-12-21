use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use structopt::StructOpt;
use thiserror::Error;

use day_ten::{Brackets, ParseBracketError};

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[derive(Debug, Error)]
enum Error {
    #[error("encountered an I/O error")]
    Io(#[from] std::io::Error),
    #[error("encountered a parsing error")]
    Parse(#[from] ParseBracketError),
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    let reader = BufReader::new(File::open(args.input)?);
    let (part1, mut part2) = reader
        .lines()
        .try_fold::<(u64, Vec<u64>), _, Result<(u64, Vec<u64>), Error>>(
            (0, vec![]),
            |(mut part1, mut part2), line| {
                let line = line.map_err(Error::Io)?;
                let brackets: Result<Brackets, _> = line.parse();

                match brackets {
                    // We don't care about valid brackets.
                    Ok(_) => {}
                    // Corruption score counts towards part 1.
                    Err(error @ ParseBracketError::Corrupt { .. }) => {
                        part1 += error.score();
                    }
                    // Incomplete score counts towards part 2.
                    Err(error @ ParseBracketError::Incomplete(_)) => {
                        part2.push(error.score());
                    }
                    // All other errors are fatal, so we stop early.
                    Err(error) => return Err(error).map_err(Error::Parse),
                }
                Ok((part1, part2))
            },
        )?;

    println!("Part 1: {}", part1);

    // Order of equal elements doesn't matter, we don't need a stable sort.
    part2.sort_unstable();
    let mid = part2.len() / 2;

    // It's possible that there are no incomplete brackets, which would make it
    // unsafe to index the median value. In that case we'll report "N/A".
    if let Some(median) = part2.get(mid) {
        println!("Part 2: {}", median);
    } else {
        println!("Part 2: N/A");
    }

    Ok(())
}
