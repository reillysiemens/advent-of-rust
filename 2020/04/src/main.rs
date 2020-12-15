use std::collections::HashMap;
// use std::fs::File;
// use std::io::{BufRead, BufReader};
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

    fn new(passport: &str) -> Result<Self, PassportError> {
        let fields = Self::fields(passport)?;
        println!("{:#?}", fields);

        let id = fields.get("pid").ok_or(PassportError::MissingField)?;
        let country_id = fields.get("cid");
        let issue_year = fields.get("iyr").ok_or(PassportError::MissingField)?;
        let expiration_year = fields.get("eyr").ok_or(PassportError::MissingField)?;
        let birth_year = fields.get("byr").ok_or(PassportError::MissingField)?;

        let height = fields.get("hgt").ok_or(PassportError::MissingField)?;
        let _hair_color = fields.get("hcl").ok_or(PassportError::MissingField)?;
        let _eye_color = fields.get("ecl").ok_or(PassportError::MissingField)?;

        Ok(Self {
            id: Self::parse_id(id)?,
            country_id: country_id.map(|cid| cid.to_string()),
            issue_year: Self::parse_issue_year(issue_year)?,
            expiration_year: Self::parse_expiration_year(expiration_year)?,
            birth_year: Self::parse_birth_year(birth_year)?,
            height: Self::parse_height(height)?,
            hair_color: "#ffffff".to_string(),
            eye_color: EyeColor::Hazel,
        })
    }
}

#[paw::main]
fn main(_args: Args) -> anyhow::Result<()> {
    Ok(())
}

#[cfg(test)]
mod test {
    use super::Passport;

    #[test]
    fn test_valid_passport() {
        let valid_passports = vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f",
            "eyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "hcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ];

        for passport in valid_passports {
            assert!(Passport::new(passport).is_ok());
        }
    }
}
