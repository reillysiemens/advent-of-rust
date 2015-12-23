use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn eval_parens(parens: &str) -> i32 {
    let mut count = 0;

    // Iterate through characters in parens string. Increment or decrement
    // accordingly. Do nothing if another character is found (i.e. '\n').
    for c in parens.chars() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => { },
        }
    }
    count
}

fn main() {
    // File opening code borrowed from
    // http://rustbyexample.com/std_misc/file/open.html.
    let input = env::args().nth(1).unwrap();

    // Create a path to the desired file
    let path = Path::new(&input);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => print!("Santa is on floor {}.\n", eval_parens(&s)),
    }

    // `file` goes out of scope, and the file gets closed
}

#[cfg(test)]
mod test {
    use super::eval_parens;

    #[test]
    fn test_zero() {
        assert_eq!(eval_parens("(())"), 0);
        assert_eq!(eval_parens("()()"), 0);
    }

    #[test]
    fn test_three() {
        assert_eq!(eval_parens("((("), 3);
        assert_eq!(eval_parens("(()(()("), 3);
        assert_eq!(eval_parens("))((((("), 3);
    }

    #[test]
    fn test_negative_one() {
        assert_eq!(eval_parens("())"), -1);
        assert_eq!(eval_parens("))("), -1);
    }

    #[test]
    fn test_negative_three() {
        assert_eq!(eval_parens(")))"), -3);
        assert_eq!(eval_parens(")())())"), -3);
    }
}
