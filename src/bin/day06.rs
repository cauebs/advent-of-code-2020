use std::{
    collections::HashSet,
    ops::{BitAnd, BitOr},
};

fn main() {
    let groups_answers = include_str!("../../inputs/day06.txt")
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|person| person.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!(
        "{}",
        groups_answers
            .iter()
            .map(|group| {
                let start = HashSet::new();
                group.iter().fold(start, |a, b| a.bitor(b)).len()
            })
            .sum::<usize>()
    );

    println!(
        "{}",
        groups_answers
            .iter()
            .map(|group| {
                let start = ('a'..='z').collect::<HashSet<_>>();
                group.iter().fold(start, |a, b| a.bitand(b)).len()
            })
            .sum::<usize>()
    );
}
