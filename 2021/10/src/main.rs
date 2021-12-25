use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use structopt::clap::AppSettings;
use structopt::StructOpt;
use thiserror::Error;

use day_ten::{Brackets, ParseBracketError};

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::ColoredHelp)]
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

struct Answer {
    part1: u64,
    part2: Vec<u64>,
}

impl Answer {
    fn new() -> Self {
        Self {
            part1: 0,
            part2: vec![],
        }
    }

    fn part1(&self) -> u64 {
        self.part1
    }

    fn part2(&mut self) -> Option<&u64> {
        // Order of equal elements doesn't matter, we don't need a stable sort.
        self.part2.sort_unstable();

        // It's possible that there are no incomplete brackets, which would
        // make it unsafe to index the median value, so we return an Option.
        let mid = self.part2.len() / 2;
        self.part2.get(mid)
    }
}

fn score(
    mut answer: Answer,
    (number, line): (usize, Result<String, std::io::Error>),
) -> Result<Answer, Error> {
    let line = line.map_err(Error::Io)?;
    let brackets: Result<Brackets, _> = line.parse();

    match brackets {
        // We don't care about valid brackets.
        Ok(_) => {}
        // Corruption score counts towards part 1.
        Err(error @ ParseBracketError::Corrupt { .. }) => {
            log::debug!("line {}: {}", number + 1, error);
            answer.part1 += error.score();
        }
        // Incomplete score counts towards part 2.
        Err(error @ ParseBracketError::Incomplete(_)) => {
            log::debug!("line {}: {}", number + 1, error);
            answer.part2.push(error.score());
        }
        // All other errors are fatal, so we stop early.
        Err(error) => return Err(error).map_err(Error::Parse),
    }
    Ok(answer)
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();

    let reader = BufReader::new(File::open(args.input)?);
    let mut answer = reader
        .lines()
        .enumerate()
        .try_fold::<Answer, _, Result<Answer, Error>>(Answer::new(), score)?;

    log::info!("Part 1: {}", answer.part1());

    if let Some(median) = answer.part2() {
        log::info!("Part 2: {}", median);
    } else {
        log::info!("Part 2: N/A");
    }

    Ok(())
}
