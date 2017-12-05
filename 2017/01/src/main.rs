use std::{env, fs, io};

fn captcha(input: &str) -> u32 {
    input
        .chars()
        .zip(input.chars().skip(1).chain(input.chars().nth(0)))
        .filter_map(|t| match (t.0, t.1) {
            (x, y) if x == y => Some(x.to_digit(10)?),
            _ => None,
        })
        .sum()
}

/// Many thanks to [Reddit](https://redd.it/32rjdd/).
fn read_input() -> Result<String, io::Error> {
    let input = env::args().nth(1).unwrap_or_else(|| "-".to_string());
    let mut reader: Box<io::Read> = if input == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(fs::File::open(input)?)
    };

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() {
    match read_input() {
        Ok(s) => println!("{}", captcha(s.trim())),
        Err(e) => println!("{:?}", e),
    }
}

#[cfg(test)]
mod test {

    use captcha;

    #[test]
    fn test_inputs() {
        let givens = vec!["1122", "1111", "1234", "91212129"];
        let expected = vec![3, 4, 0, 9];
        let actuals: Vec<u32> = givens.iter().map(|g| captcha(&g)).collect();

        assert_eq!(expected, actuals);

    }
}
