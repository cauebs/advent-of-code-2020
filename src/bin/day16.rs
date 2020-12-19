#![feature(str_split_once, hash_drain_filter)]

use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
    ops::RangeInclusive,
    str::FromStr,
};

struct TicketRule {
    field_label: String,
    valid_ranges: Vec<RangeInclusive<u32>>,
}

impl FromStr for TicketRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (field_label, valid_ranges) = s.split_once(": ").ok_or(())?;

        let valid_ranges = valid_ranges
            .split(" or ")
            .map(|r| {
                let (min, max) = r.split_once("-").unwrap();
                min.parse().unwrap()..=max.parse().unwrap()
            })
            .collect();

        Ok(Self {
            field_label: field_label.to_owned(),
            valid_ranges,
        })
    }
}

impl TicketRule {
    fn is_in_range(&self, value: u32) -> bool {
        self.valid_ranges.iter().any(|r| r.contains(&value))
    }
}

#[derive(Clone)]
struct Ticket {
    values: Vec<u32>,
}

impl FromStr for Ticket {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()
            .map(|values| Self { values })
    }
}

fn error_rate(tickets: &[Ticket], rules: &[TicketRule]) -> u32 {
    let invalid_values = tickets.iter().flat_map(|ticket| {
        ticket
            .values
            .iter()
            .filter(|&&value| !rules.iter().any(|rule| rule.is_in_range(value)))
    });

    invalid_values.sum()
}

fn solve(tickets: &[Ticket], rules: &[TicketRule]) -> HashMap<usize, String> {
    let mut fields = (0..rules.len())
        .map(|i| {
            let all_values_match = |rule: &TicketRule| {
                tickets
                    .iter()
                    .all(|ticket| rule.is_in_range(ticket.values[i]))
            };

            let possible_labels = rules
                .iter()
                .filter(|rule| all_values_match(rule))
                .map(|rule| rule.field_label.clone())
                .collect::<HashSet<_>>();

            (i, possible_labels)
        })
        .collect::<HashMap<_, _>>();

    let mut fixed = HashMap::new();

    loop {
        let new_fixed = fields
            .drain_filter(|_i, possible_labels| possible_labels.len() == 1)
            .map(|(i, mut possible_labels)| (i, possible_labels.drain().next().unwrap()))
            .collect::<HashMap<_, _>>();

        fixed.extend(new_fixed.clone());

        if new_fixed.is_empty() {
            break;
        }

        for possible_labels in fields.values_mut() {
            for fixed in new_fixed.values() {
                possible_labels.remove(fixed);
            }
        }
    }

    fixed
}

fn main() {
    let (rules, tickets) = include_str!("../../inputs/day16.txt")
        .split_once("\n\n")
        .unwrap();

    let rules: Vec<TicketRule> = rules
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    let (my_ticket, nearby_tickets) = tickets.split_once("\n\n").unwrap();

    let my_ticket: Ticket = my_ticket
        .lines()
        .nth(1)
        .and_then(|l| l.parse().ok())
        .unwrap();

    let nearby_tickets: Vec<Ticket> = nearby_tickets
        .lines()
        .skip(1)
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    println!("{}", error_rate(&nearby_tickets, &rules));

    let maybe_valid = nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket
                .values
                .iter()
                .all(|&value| rules.iter().any(|rule| rule.is_in_range(value)))
        })
        .cloned()
        .collect::<Vec<_>>();

    let field_mapping = solve(&maybe_valid, &rules);

    println!(
        "{}",
        field_mapping
            .iter()
            .filter(|(_i, label)| label.starts_with("departure"))
            .map(|(&i, _)| u64::from(my_ticket.values[i]))
            .product::<u64>()
    );
}
