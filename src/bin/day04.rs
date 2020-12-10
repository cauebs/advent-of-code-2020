use if_chain::if_chain;
use regex::Regex;

use std::collections::HashMap;

#[derive(Debug)]
struct Passport<'a>(HashMap<&'a str, &'a str>);

#[derive(Debug)]
enum PassportError<'a> {
    MissingRequiredField,
    InvalidFieldValue(&'a str),
}
use PassportError::*;

impl<'a> Passport<'a> {
    fn has_required_fields(&self) -> bool {
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|f| self.0.contains_key(f))
    }

    fn validate(&self) -> Result<(), PassportError> {
        let get_field = |name: &str| self.0.get(name).ok_or(MissingRequiredField);
        let parse_field = |name: &'a str| {
            get_field(name).and_then(|value| value.parse().map_err(|_| InvalidFieldValue(name)))
        };

        let birth_year: u32 = parse_field("byr")?;
        let issue_year: u32 = parse_field("iyr")?;
        let expiration_year: u32 = parse_field("eyr")?;

        let height = get_field("hgt")?;
        let cap = Regex::new(r"^(?P<value>\d+)(?P<unit>(cm)|(in))$")
            .unwrap()
            .captures(height)
            .ok_or(InvalidFieldValue("hgt"))?;

        let height_value = cap["value"]
            .parse::<u32>()
            .map_err(|_| InvalidFieldValue("hgt"))?;

        let height_unit = cap["unit"].to_owned();

        let color_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        let id_regex = Regex::new(r"^\d{9}$").unwrap();

        if_chain! {
            if (1920..=2002).contains(&birth_year);
            if (2010..=2020).contains(&issue_year);
            if (2020..=2030).contains(&expiration_year);

            if (height_unit == "cm" && (150..=193).contains(&height_value))
            || (height_unit == "in" && (59..=76).contains(&height_value));

            if color_regex.is_match(get_field("hcl")?);
            if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(get_field("ecl")?);
            if id_regex.is_match(get_field("pid")?);

            then { Ok(()) } else { Err(InvalidFieldValue("")) }
        }
    }
}

fn main() {
    let input = include_str!("../../inputs/day4.txt");

    let field_regex = Regex::new(r"^(?P<name>\w{3}):(?P<value>.+)$").unwrap();

    let passports: Vec<Passport> = input
        .split("\n\n")
        .map(|p| {
            p.split_whitespace()
                .map(|field| {
                    let cap = field_regex.captures(field).unwrap();
                    (
                        cap.name("name").unwrap().as_str(),
                        cap.name("value").unwrap().as_str(),
                    )
                })
                .collect()
        })
        .map(Passport)
        .collect();

    println!(
        "{}",
        passports.iter().filter(|p| p.has_required_fields()).count()
    );

    println!(
        "{:#?}",
        passports.iter().filter(|p| p.validate().is_ok()).count()
    );
}
