use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num;

#[derive(Debug)]
enum Error {
    MissingArgument,
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

fn fuel(mass: &usize) -> usize {
    (mass / 3) - 2
}

fn main() -> Result<(), Error> {
    let input = env::args().nth(1).ok_or(Error::MissingArgument)?;
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let masses = reader
        .lines()
        .map(|line| {
            line.map_err(Error::IoError)?
                .parse()
                .map_err(Error::ParseError)
        })
        .collect::<Result<Vec<usize>, Error>>()?;

    let mass_only: usize = masses.iter().map(fuel).sum();

    println!("Part 1: {}", mass_only);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::fuel;

    #[test]
    fn fuel_handles_example_inputs() {
        let inputs = vec![
            (&12usize, 2usize),
            (&14usize, 2usize),
            (&1_969usize, 654usize),
            (&100_756usize, 33_583usize),
        ];

        for (given, expected) in inputs {
            assert_eq!(expected, fuel(given));
        }
    }
}
