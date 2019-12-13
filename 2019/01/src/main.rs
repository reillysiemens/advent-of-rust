use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num;

#[derive(Debug)]
enum Error {
    MissingArgument,
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

/// Determine total fuel required for the given mass.
fn fuel(mass: &isize) -> isize {
    (mass / 3) - 2
}

/// Include fuel costs in determining total fuel required for the given mass.
fn factor_fuel(mass: &isize) -> isize {
    let fuel = (mass / 3) - 2;
    match fuel <= 0 {
        true => 0,
        false => fuel + factor_fuel(&fuel),
    }
}

fn main() -> Result<(), Error> {
    let input = env::args().nth(1).ok_or(Error::MissingArgument)?;
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let results: Result<(isize, isize), Error> =
        reader.lines().try_fold((0, 0), |(f, ff), line| {
            let mass = line.map_err(Error::Io)?.parse().map_err(Error::Parse)?;
            Ok((f + fuel(&mass), ff + factor_fuel(&mass)))
        });

    let (part1, part2) = results?;
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{factor_fuel, fuel};

    #[test]
    fn fuel_handles_example_inputs() {
        let inputs = vec![
            (&12isize, 2isize),
            (&14isize, 2isize),
            (&1_969isize, 654isize),
            (&100_756isize, 33_583isize),
        ];

        for (given, expected) in inputs {
            assert_eq!(expected, fuel(given));
        }
    }

    #[test]
    fn factor_fuel_handles_example_inputs() {
        let inputs = vec![
            (&12isize, 2isize),
            (&14isize, 2isize),
            (&1_969isize, 966isize),
            (&100_756isize, 50_346isize),
        ];

        for (given, expected) in inputs {
            assert_eq!(expected, factor_fuel(given));
        }
    }
}
