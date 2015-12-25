use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn visit_houses(directions: &str) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut houses = vec![];
    houses.push((x, y));
    for c in directions.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => {}
        }

        if !houses.contains(&(x, y)) {
            houses.push((x, y));
        }
    }
    houses.len()
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
            let at_least_one_present = visit_houses(&s);
            println!("{} houses receive at least one present.",
                     at_least_one_present);
        }
    }
}

#[cfg(test)]
mod test {
    use super::visit_houses;

    #[test]
    fn test_directions() {
        assert_eq!(2, visit_houses(">"));
        assert_eq!(4, visit_houses("^>v<"));
        assert_eq!(2, visit_houses("^v^v^v^v^v"));
    }
}
