#![feature(try_trait)]
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num;
use std::ops::Try;

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

impl From<num::ParseIntError> for Error {
    fn from(error: num::ParseIntError) -> Self {
        Error::Parse(error)
    }
}

struct TryMap<I, F> {
    iter: I,
    f: F,
}

impl<T, E, I, F> Iterator for TryMap<I, F>
where
    I: Iterator,
    <I as Iterator>::Item: Try<Ok = T, Error = E>,
    F: FnMut(T) -> Result<T, E>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

trait TryMapExt {
    type Item;
    type Error;

    fn try_map<T, R, F>(self, f: F) -> TryMap<Self, F>
    where
        Self: Sized,
        R: Try<Ok = T, Error = Self::Error>,
        F: FnMut(Self::Item) -> R,
    {
        TryMap { iter: self, f }
    }
}

impl<I, T, E> TryMapExt for I
where
    I: Iterator<Item = Result<T, E>>,
{
    type Item = T;
    type Error = E;
}

fn fuel(mass: isize) -> isize {
    (mass / 3) - 2
}

fn fuel_factored(mass: isize) -> isize {
    3
}

fn main() -> Result<(), Error> {
    let input = env::args().nth(1).ok_or(Error::MissingArgument)?;
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let fuel = reader
        .lines()
        .try_map(|line| line.parse())
        .fold((0, 0), |(f, ff), mass| {
            (f + fuel(mass), fuel_factored(mass))
        });

    println!("Part 1: {}", fuel);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{fuel, precise_fuel};

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
            assert_eq!(expected, precise_fuel(given));
        }
    }
}
