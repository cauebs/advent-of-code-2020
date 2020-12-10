use regex::Regex;

use std::str::FromStr;

struct Entry {
    numbers: (usize, usize),
    letter: char,
    password: String,
}

struct EntryParseError;

impl FromStr for Entry {
    type Err = EntryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = r"^(?P<n1>\d+)-(?P<n2>\d+) (?P<letter>[a-z]): (?P<password>[a-z]+)$";
        let re = Regex::new(pattern).unwrap();
        let cap = re.captures(s).ok_or(EntryParseError)?;

        Ok(Self {
            numbers: (cap["n1"].parse().unwrap(), cap["n2"].parse().unwrap()),
            letter: cap["letter"].chars().next().unwrap(),
            password: cap["password"].to_owned(),
        })
    }
}

trait Policy {
    fn check(&self, entry: &Entry) -> bool;
}

struct SledRentalPolicy;

impl Policy for SledRentalPolicy {
    fn check(&self, entry: &Entry) -> bool {
        let num_occurrences = entry
            .password
            .chars()
            .filter(|c| *c == entry.letter)
            .count();

        let (min, max) = entry.numbers;
        (min..=max).contains(&num_occurrences)
    }
}

struct TobogganCorporatePolicy;

impl Policy for TobogganCorporatePolicy {
    fn check(&self, entry: &Entry) -> bool {
        let letter1 = entry.password.chars().nth(entry.numbers.0 - 1).unwrap();
        let letter2 = entry.password.chars().nth(entry.numbers.1 - 1).unwrap();

        (letter1 == entry.letter) ^ (letter2 == entry.letter)
    }
}

fn main() {
    let entries = include_str!("../../inputs/day2.txt")
        .lines()
        .filter_map(|line| line.parse::<Entry>().ok())
        .collect::<Vec<_>>();

    println!(
        "{}",
        entries
            .iter()
            .filter(|entry| SledRentalPolicy.check(entry))
            .count()
    );

    println!(
        "{}",
        entries
            .iter()
            .filter(|entry| TobogganCorporatePolicy.check(entry))
            .count()
    );
}
