#![feature(iter_arith)]

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn split_as_u32(s: &str, c: char) -> Vec<u32> {
    s.split(c)
     .map(|x| {
         x.parse::<u32>()
          .ok()
          .unwrap()
     })
     .collect()
}

fn materials(dimensions: &str) -> (u32, u32) {
    let mut d = split_as_u32(dimensions, 'x');
    d.sort();
    let smallest = &d[..2];
    let ribbon = d.iter().product::<u32>() + smallest.iter().map(|x| 2 * x).sum::<u32>();
    let sides = vec![(d[0] * d[1]), (d[1] * d[2]), (d[2] * d[0])];
    let surface_area = sides.iter().map(|x| 2 * x).sum::<u32>();
    (surface_area + sides[0], ribbon)
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
            let (mut surface_area, mut ribbon) = (0, 0);
            for line in s.lines() {
                let (sa, r) = materials(line);
                surface_area += sa;
                ribbon += r;
            }
            println!("Santa's elves need {} total square feet of wrapping paper.\nThey also need \
                      {} total feet of ribbon.",
                     surface_area,
                     ribbon);
        }
    }
}

#[cfg(test)]
mod test {
    use super::materials;

    #[test]
    fn test_two_by_three_by_four() {
        let (surface_area, ribbon) = materials("2x3x4");
        assert_eq!(58, surface_area);
        assert_eq!(34, ribbon);
    }

    #[test]
    fn test_one_by_one_by_ten() {
        let (surface_area, ribbon) = materials("1x1x10");
        assert_eq!(43, surface_area);
        assert_eq!(14, ribbon);
    }
}
