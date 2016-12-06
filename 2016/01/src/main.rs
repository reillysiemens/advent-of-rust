use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;

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

fn interpolate(old: (i64, i64), new: (i64, i64)) -> Vec<(i64, i64)> {
    // Note: NOT OK to iterate backwards.
    let (x1, y1) = old;
    let (x2, y2) = new;


    // The two x coordinates are equal, so we interpolate with y coordinates.
    if x1 == x2 {
        if y1 < y2 { (y1..y2) } else { (y2..y1) }
            .map(|i| (x1, i))
            .collect()

    // The two y coordinates are equal, so we interpolate with x coordinates.
    } else {
        if x1 < x2 { (x1..x2) } else { (x2..x1) }
            .map(|i| (i, y1))
            .collect()
    }
}

fn navigate(instructions: Vec<Instruction>) -> (i64, i64) {
    let mut location = (0i64, 0i64);
    let mut previous_location: (i64, i64);
    let mut direction = Direction::North;

    let mut found = false;
    let mut actual_distance = 0;
    let mut locations = HashSet::new();

    // Record the first location in case we come back to it!
    // locations.insert(location.clone());

    for instruction in instructions {

        let turn = match &instruction.turn[..] {
            "L" => Turn::Left(direction),
            "R" => Turn::Right(direction),
            _ => panic!("Invalid instruction received"),
        };

        println!("Traveling to: {:?}", location);

        previous_location = location;

        match turn {
            Turn::Left(Direction::North) |
            Turn::Right(Direction::South) => {
                location.0 -= instruction.blocks;
                direction = Direction::West;
            }
            Turn::Right(Direction::North) |
            Turn::Left(Direction::South) => {
                location.0 += instruction.blocks;
                direction = Direction::East;
            }
            Turn::Left(Direction::East) |
            Turn::Right(Direction::West) => {
                location.1 += instruction.blocks;
                direction = Direction::North;
            }
            Turn::Right(Direction::East) |
            Turn::Left(Direction::West) => {
                location.1 -= instruction.blocks;
                direction = Direction::South;
            }
        }

        if !found {
            println!("{:?}, {:?}", previous_location, location);
            let coordinates = interpolate(previous_location, location);
            println!("Interpolated: {:?}", coordinates.clone());
            for coord in coordinates {
                if locations.contains(&coord) {
                    actual_distance = location.0.abs() + location.1.abs();
                    found = true;
                    println!("Found actual location at {:?}", location);
                } else {
                    locations.insert(coord);
                };
            }
        };
    }
    println!("Accumulated locations: {:?}", locations);
    (location.0.abs() + location.1.abs(), actual_distance)
}

fn main() {
    let input = match env::args().nth(1) {
        Some(input) => input,
        None => panic!("Please give an input file"),
    };

    let path = Path::new(&input);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => {
            panic!("Couldn't open {}: {}", display, Error::description(&why));
        }
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            panic!("Couldn't read {}: {}", display, Error::description(&why));
        }
        Ok(_) => {
            let instructions = parse_instructions(&s.trim());
            let results = navigate(instructions);
            println!("Easter Bunny HQ is {} blocks away.", results.0);
            println!("Oh, wait... nope, it's {} blocks away!", results.1);
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
