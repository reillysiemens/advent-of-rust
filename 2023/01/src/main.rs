mod part1;
mod part2;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    input: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let reader = BufReader::new(File::open(args.input)?);
    let lines = reader.lines().collect::<Result<Vec<String>, _>>()?;
    let part1 = part1::solve(&lines)?;
    let part2 = part2::solve(&lines)?;
    println!("Part 1: {part1}\nPart 2: {part2}");
    Ok(())
}
