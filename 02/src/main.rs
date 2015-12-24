#![feature(iter_arith)]

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn split_as_u32(s: &str, c: char) -> (u32, u32, u32) {
    let n: Vec<u32> = s.split(c)
                       .map(|x| {
                           x.parse::<u32>()
                            .ok()
                            .unwrap()
                       })
                       .collect();
    (n[0], n[1], n[2])
}

fn total_area(dimensions: &str) -> u32 {
    let (l, w, h) = split_as_u32(dimensions, 'x');
    let sides = vec![(l * w), (w * h), (h * l)];
    let surface_area = sides.iter().map(|x| 2 * x).sum::<u32>();
    let smallest_side = sides.iter().min().unwrap();
    surface_area + smallest_side
}

fn main() {
    let input = env::args().nth(1).unwrap();

    let path = Path::new(&input);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => {
            let answer: u32 = s.lines()
                               .map(|line| total_area(line))
                               .sum();
            println!("The elves need {} total square feet of wrapping paper.",
                     answer);
        }
    }
}

#[cfg(test)]
mod test {
    use super::total_area;

    #[test]
    fn test_two_by_three_by_four() {
        assert_eq!(58, total_area("2x3x4"));
    }

    #[test]
    fn test_one_by_one_by_ten() {
        assert_eq!(43, total_area("1x1x10"));
    }
}
