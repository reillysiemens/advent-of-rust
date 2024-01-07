use std::{fs, num::ParseIntError, path::PathBuf};

use anyhow::anyhow;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    input: PathBuf,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Invalid opcode: {0}")]
    Opcode(u64),
}

fn parse(code: &str) -> Result<Vec<u64>, ParseIntError> {
    code.trim()
        .split(',')
        .map(|c| c.parse::<u64>())
        .collect::<Result<Vec<_>, ParseIntError>>()
}

fn run(mut memory: Vec<u64>) -> Result<Vec<u64>, Error> {
    let mut pointer = 0;
    loop {
        // Note: Indexing could panic if given an invalid program.
        let opcode = memory[pointer];
        match opcode {
            1 => {
                let address1 = memory[pointer + 1] as usize;
                let address2 = memory[pointer + 2] as usize;
                let address3 = memory[pointer + 3] as usize;

                let left_addend = memory[address1];
                let right_addend = memory[address2];
                memory[address3] = left_addend + right_addend;
            }
            2 => {
                let address1 = memory[pointer + 1] as usize;
                let address2 = memory[pointer + 2] as usize;
                let address3 = memory[pointer + 3] as usize;

                let multiplier = memory[address1];
                let multiplicand = memory[address2];
                memory[address3] = multiplier * multiplicand;
            }
            99 => break,
            _ => return Err(Error::Opcode(opcode)),
        }
        pointer += 4;
    }
    Ok(memory)
}

fn part1(mut program: Vec<u64>) -> anyhow::Result<u64> {
    program[1] = 12;
    program[2] = 2;
    let memory = run(program)?;
    Ok(memory[0])
}

fn part2(program: Vec<u64>) -> anyhow::Result<u64> {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = program.clone();
            memory[1] = noun;
            memory[2] = verb;
            if run(memory)?[0] == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err(anyhow!("No noun and verb which yield 19690720"))
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let code = fs::read_to_string(args.input)?;
    let program = parse(&code)?;

    println!("Part 1: {}", part1(program.clone())?);
    println!("Part 2: {}", part2(program)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    use super::*;

    #[test]
    fn parsing() -> anyhow::Result<()> {
        let code = "1,9,10,3,2,3,11,0,99,30,40,50";
        let expected = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let program = parse(code)?;

        assert_eq!(program, expected);
        Ok(())
    }

    #[test_case(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50] ; "program 1")]
    #[test_case(vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99] ; "program 2")]
    #[test_case(vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99] ; "program 3")]
    #[test_case(vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801] ; "program 4")]
    #[test_case(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], vec![30, 1, 1, 4, 2, 5, 6, 0, 99] ; "program 5")]
    fn running(program: Vec<u64>, expected: Vec<u64>) -> anyhow::Result<()> {
        let memory = run(program)?;
        assert_eq!(memory, expected);
        Ok(())
    }
}
