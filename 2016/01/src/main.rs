use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Debug)]
enum Turn {
    Left(Direction),
    Right(Direction),
}

#[derive(Debug)]
struct Instruction {
    turn: String,
    blocks: i64,
}

fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    instructions.split(", ")
        .map(|string| {
            let (turn, blocks) = string.split_at(1);
            Instruction {
                turn: turn.to_string(),
                blocks: blocks.parse().unwrap(),
            }
        })
        .collect()
}

fn navigate(instructions: Vec<Instruction>) -> i64 {
    let (mut x, mut y) = (0, 0);
    let mut direction = Direction::North;

    for instruction in instructions {

        let turn = match &instruction.turn[..] {
            "L" => Turn::Left(direction),
            "R" => Turn::Right(direction),
            _ => panic!("Invalid instruction received"),
        };

        match turn {
            Turn::Left(Direction::North) | Turn::Right(Direction::South) => {
                x -= instruction.blocks;
                direction = Direction::West;
            },
            Turn::Right(Direction::North) | Turn::Left(Direction::South) => {
                x += instruction.blocks;
                direction = Direction::East;
            },
            Turn::Left(Direction::East) | Turn::Right(Direction::West) => {
                y += instruction.blocks;
                direction = Direction::North;
            },
            Turn::Right(Direction::East) | Turn::Left(Direction::West) => {
                y -= instruction.blocks;
                direction = Direction::South;
            },
        }

        println!("{:?}: ({}, {}) {:?}", direction, x, y, instruction);
    }
    x.abs() + y.abs()
}

fn main() {
    let input = env::args().nth(1).unwrap();

    let path = Path::new(&input);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => {
            panic!("Couldn't open {}: {}", display, Error::description(&why));
        },
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            panic!("Couldn't read {}: {}", display, Error::description(&why));
        },
        Ok(_) => {
            let instructions = parse_instructions(&s.trim());
            let result = navigate(instructions);
            println!("Easter Bunny HQ is {} blocks away.", result);
        }
    };
}

#[cfg(test)]
mod test {
    use super::{parse_instructions, navigate};

    #[test]
    fn test_navigation() {

        let expectations = vec![
            (5, parse_instructions("R2, L3")),
            (2, parse_instructions("R2, R2, R2")),
            (12, parse_instructions("R5, L5, R5, R3")),
        ];

        for (result, instructions) in expectations {
            assert_eq!(result, navigate(instructions));
        }
    }
}
