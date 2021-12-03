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

fn most_common_values(report: &[String]) -> Vec<char> {
    let length = report.len();
    let width = report[0].len();
    let mut columns = vec![0; width];

    for number in report {
        for (idx, bit) in number.chars().enumerate() {
            columns[idx] += if bit == '1' { 1 } else { 0 }
        }
    }

    let values = columns
        .iter()
        .map(|value| if value >= &(length / 2) { '1' } else { '0' })
        .collect();

    values
}

fn flip(values: &[char]) -> Vec<char> {
    values
        .iter()
        .map(|value| if *value == '1' { '0' } else { '1' })
        .collect()
}

fn power_consumption(report: &[String]) -> Result<u32, ParseIntError> {
    if report.len() == 0 {
        return Ok(0);
    }

    let most_common = most_common_values(&report);
    let epsilon = flip(&most_common).iter().collect::<String>();
    let gamma = most_common.iter().collect::<String>();

    Ok(u32::from_str_radix(&gamma, 2)? * u32::from_str_radix(&epsilon, 2)?)
}

fn find_rating(report: &[String], criteria: &[char]) -> Result<u32, ParseIntError> {
    let mut ratings = report.to_vec();
    let mut candidates = vec![];

    for (idx, value) in criteria.iter().enumerate() {
        for rating in &ratings {
            if rating.chars().nth(idx).unwrap() == *value {
                candidates.push(rating.clone());
            }
        }

        if candidates.len() == 1 {
            return Ok(u32::from_str_radix(&ratings[0].clone(), 2)?);
        } else {
            ratings = candidates.clone();
            candidates.clear();
        }
    }

    Ok(42) // XXX This is clearly not right.
}

fn life_support_rating(report: &[String]) -> Result<u32, ParseIntError> {
    let most_common = most_common_values(&report);
    let _least_common = flip(&most_common);
    Ok(0)
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    let reader = BufReader::new(File::open(args.input).map_err(Error::Io)?);
    let report = reader.lines().collect::<Result<Vec<String>, _>>()?;
    println!("Part 1: {}", power_consumption(&report)?);

    Ok(())
}
