use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[derive(PartialEq, Debug)]
enum Height {
    Inches(u32),
    Centimeters(u32),
}

#[derive(PartialEq, Debug)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

#[derive(PartialEq, Debug)]
enum PassportError {
    InvalidID,
    InvalidIssueYear,
    InvalidExpirationYear,
    InvalidBirthYear,
    InvalidHeight,
    InvalidHairColor,
    InvalidEyeColor,
    InvalidField,
    MissingField,
}

#[derive(PartialEq, Debug)]
struct Passport {
    /// Passport ID: A nine-digit number, including leading zeroes.
    id: String,
    /// Country ID: Ignored, missing or not.
    country_id: Option<String>,
    /// Four-digit year; at least 2010 and at most 2020.
    issue_year: u32,
    /// Four-digit year; at least 2020 and at most 2030.
    expiration_year: u32,
    /// Four-digit year; at least 1920 and at most 2002.
    birth_year: u32,
    /// Height: centimeters (between 150-193cm) or inches (between 59-76in).
    height: Height,
    /// Hair color: six-digit hexadecimal color.
    hair_color: String,
    /// Eye color: Amber, blue, brown, gray, green, hazel, or other.
    eye_color: EyeColor,
}

impl Passport {
    fn fields(passport: &str) -> Result<HashMap<&str, &str>, PassportError> {
        passport
            .split_whitespace()
            .map(|field| {
                let mut field = field.split(':');
                match (field.next(), field.next()) {
                    (Some(key), Some(value)) => Ok((key, value)),
                    _ => Err(PassportError::InvalidField),
                }
            })
            .collect()
    }

    fn parse_id(id: &str) -> Result<String, PassportError> {
        // Parse to validate as a number, but ignore the result.
        match (id.len(), id.parse::<u32>()) {
            (9, Ok(_)) => Ok(id.to_string()),
            _ => Err(PassportError::InvalidID),
        }
    }

    fn parse_issue_year(issue_year: &str) -> Result<u32, PassportError> {
        issue_year
            .parse::<u32>()
            .map_err(|_| PassportError::InvalidIssueYear)
            .and_then(|iyr| match iyr {
                2010..=2020 => Ok(iyr),
                _ => Err(PassportError::InvalidIssueYear),
            })
    }

    fn parse_expiration_year(expiration_year: &str) -> Result<u32, PassportError> {
        expiration_year
            .parse::<u32>()
            .map_err(|_| PassportError::InvalidExpirationYear)
            .and_then(|eyr| match eyr {
                2020..=2030 => Ok(eyr),
                _ => Err(PassportError::InvalidExpirationYear),
            })
    }

    fn parse_birth_year(birth_year: &str) -> Result<u32, PassportError> {
        birth_year
            .parse::<u32>()
            .map_err(|_| PassportError::InvalidBirthYear)
            .and_then(|byr| match byr {
                1920..=2002 => Ok(byr),
                _ => Err(PassportError::InvalidBirthYear),
            })
    }

    fn parse_height(height: &str) -> Result<Height, PassportError> {
        if let Some(idx) = height.rfind("cm") {
            let height = height[..idx]
                .parse()
                .map_err(|_| PassportError::InvalidHeight)?;
            return match (150..=193).contains(&height) {
                true => Ok(Height::Centimeters(height)),
                false => Err(PassportError::InvalidHeight),
            };
        }

        if let Some(idx) = height.rfind("in") {
            let height = height[..idx]
                .parse()
                .map_err(|_| PassportError::InvalidHeight)?;
            return match (59..=76).contains(&height) {
                true => Ok(Height::Inches(height)),
                false => Err(PassportError::InvalidHeight),
            };
        }

        Err(PassportError::InvalidHeight)
    }

    fn parse_hair_color(hair_color: &str) -> Result<String, PassportError> {
        match (hair_color.len(), hair_color.starts_with('#')) {
            (7, true) => {
                u32::from_str_radix(&hair_color[1..], 16)
                    .map_err(|_| PassportError::InvalidHairColor)?;
                Ok(hair_color.to_string())
            }
            _ => Err(PassportError::InvalidHairColor),
        }
    }

    fn parse_eye_color(eye_color: &str) -> Result<EyeColor, PassportError> {
        match eye_color {
            "amb" => Ok(EyeColor::Amber),
            "blu" => Ok(EyeColor::Blue),
            "brn" => Ok(EyeColor::Brown),
            "gry" => Ok(EyeColor::Gray),
            "grn" => Ok(EyeColor::Green),
            "hzl" => Ok(EyeColor::Hazel),
            "oth" => Ok(EyeColor::Other),
            _ => Err(PassportError::InvalidEyeColor),
        }
    }

    fn new(passport: &str) -> Result<Self, PassportError> {
        let fields = Self::fields(passport)?;
        println!("{:#?}", fields);

        let id = fields.get("pid").ok_or(PassportError::MissingField)?;
        let country_id = fields.get("cid");
        let issue_year = fields.get("iyr").ok_or(PassportError::MissingField)?;
        let expiration_year = fields.get("eyr").ok_or(PassportError::MissingField)?;
        let birth_year = fields.get("byr").ok_or(PassportError::MissingField)?;
        let height = fields.get("hgt").ok_or(PassportError::MissingField)?;
        let hair_color = fields.get("hcl").ok_or(PassportError::MissingField)?;
        let eye_color = fields.get("ecl").ok_or(PassportError::MissingField)?;

        Ok(Self {
            id: Self::parse_id(id)?,
            country_id: country_id.map(|cid| cid.to_string()),
            issue_year: Self::parse_issue_year(issue_year)?,
            expiration_year: Self::parse_expiration_year(expiration_year)?,
            birth_year: Self::parse_birth_year(birth_year)?,
            height: Self::parse_height(height)?,
            hair_color: Self::parse_hair_color(hair_color)?,
            eye_color: Self::parse_eye_color(eye_color)?,
        })
    }
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    let mut part1 = 0;
    let mut part2 = 0;
    let mut string = String::new();

    let reader = BufReader::new(File::open(args.input)?);
    let mut lines = reader.lines();

    loop {
        match lines.next() {
            Some(line) => match line?.as_ref() {
                "" => {
                    match Passport::new(string.as_ref()) {
                        Ok(_) => {
                            part1 += 1;
                            part2 += 1;
                        }
                        Err(PassportError::MissingField) => {}
                        Err(_) => part1 += 1,
                    }
                    println!("{}", string.clone());
                    string.clear();
                }
                line => {
                    string.push_str(line);
                    string.push('\n');
                }
            },
            None => {
                match Passport::new(string.as_ref()) {
                    Ok(_) => {
                        part1 += 1;
                        part2 += 1;
                    }
                    Err(PassportError::MissingField) => {}
                    Err(_) => part1 += 1,
                }
                string.clear();
                break;
            }
        }
    }

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{EyeColor, Height, Passport, PassportError};

    #[test]
    fn test_valid_passports() {
        let valid_passports = vec![
            (
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f",
                Passport {
                    id: "087499704".to_string(),
                    country_id: None,
                    issue_year: 2012,
                    expiration_year: 2030,
                    birth_year: 1980,
                    height: Height::Inches(74),
                    hair_color: "#623a2f".to_string(),
                    eye_color: EyeColor::Green,
                },
            ),
            (
                "eyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
                Passport {
                    id: "896056539".to_string(),
                    country_id: Some("129".to_string()),
                    issue_year: 2014,
                    expiration_year: 2029,
                    birth_year: 1989,
                    height: Height::Centimeters(165),
                    hair_color: "#a97842".to_string(),
                    eye_color: EyeColor::Blue,
                },
            ),
            (
                "hcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022",
                Passport {
                    id: "545766238".to_string(),
                    country_id: Some("88".to_string()),
                    issue_year: 2015,
                    expiration_year: 2022,
                    birth_year: 2001,
                    height: Height::Centimeters(164),
                    hair_color: "#888785".to_string(),
                    eye_color: EyeColor::Hazel,
                },
            ),
            (
                "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
                Passport {
                    id: "093154719".to_string(),
                    country_id: None,
                    issue_year: 2010,
                    expiration_year: 2021,
                    birth_year: 1944,
                    height: Height::Centimeters(158),
                    hair_color: "#b6652a".to_string(),
                    eye_color: EyeColor::Blue,
                },
            ),
        ];

        for (passport, expected) in valid_passports {
            assert_eq!(Passport::new(passport).unwrap(), expected);
        }
    }

    #[test]
    fn test_invalid_passports() {
        let invalid_passports = vec![
            (
                "eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
                Err(PassportError::InvalidID),
            ),
            (
                "iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946",
                Err(PassportError::InvalidExpirationYear),
            ),
            (
                "hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
                Err(PassportError::InvalidHairColor),
            ),
            (
                "hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007",
                Err(PassportError::InvalidID),
            ),
        ];
        for (passport, expected) in invalid_passports {
            assert_eq!(Passport::new(passport), expected);
        }
    }
}
