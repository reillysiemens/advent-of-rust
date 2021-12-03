use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
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
}

fn power_consumption(report: &[String]) -> Result<u32, ParseIntError> {
    let length = report.len();
    if length == 0 {
        return Ok(0);
    }

    let width = report[0].len();
    let mut columns = vec![0; width];

    for number in report {
        for (idx, bit) in number.chars().enumerate() {
            columns[idx] += if bit == '1' { 1 } else { 0 }
        }
    }

    let gamma = columns
        .iter()
        .map(|value| if value > &(length / 2) { '1' } else { '0' })
        .collect::<String>();
    let epsilon = gamma
        .chars()
        .map(|bit| if bit == '1' { '0' } else { '1' })
        .collect::<String>();

    Ok(u32::from_str_radix(&gamma, 2)? * u32::from_str_radix(&epsilon, 2)?)
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    let reader = BufReader::new(File::open(args.input).map_err(Error::Io)?);
    let report = reader.lines().collect::<Result<Vec<String>, _>>()?;
    println!("Part 1: {}", power_consumption(&report)?);

    Ok(())
}
