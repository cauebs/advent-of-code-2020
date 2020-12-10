#![feature(str_split_once)]

use std::collections::{HashMap, HashSet};

type Color<'a> = &'a str;

fn parse_rule(s: &str) -> Option<(&str, HashMap<Color, u32>)> {
    let (color, allowed_contents) = s.split_once(" bags contain ")?;

    if allowed_contents == "no other bags." {
        return Some((color, HashMap::new()));
    }

    Some((
        color,
        allowed_contents
            .split(", ")
            .filter_map(parse_content)
            .collect(),
    ))
}

fn parse_content(s: &str) -> Option<(Color, u32)> {
    let (amount, bag) = s.split_once(" ")?;
    let (color, _) = bag.rsplit_once(" ")?;

    Some((color, amount.parse().ok()?))
}

type Rules<'a> = HashMap<&'a str, HashMap<&'a str, u32>>;

fn direct_allowed_containers<'a>(color: Color<'a>, rules: &Rules<'a>) -> HashSet<Color<'a>> {
    rules
        .iter()
        .filter(|(_, allowed_contents)| allowed_contents.contains_key(&color))
        .map(|(container_color, _)| *container_color)
        .collect()
}

fn allowed_containers<'a>(color: Color<'a>, rules: &Rules<'a>) -> HashSet<Color<'a>> {
    let mut containers = direct_allowed_containers(color, rules);
    let mut unchecked_colors = containers.clone();

    while !unchecked_colors.is_empty() {
        let new_colors = unchecked_colors
            .iter()
            .flat_map(|color| direct_allowed_containers(color, rules).into_iter())
            .filter(|color| !containers.contains(*color))
            .collect();

        containers.extend(&new_colors);
        unchecked_colors = new_colors;
    }

    containers
}

fn count_contents<'a>(color: Color<'a>, rules: &Rules<'a>) -> u32 {
    rules[color]
        .iter()
        .map(|(color, amount)| amount + amount * count_contents(color, rules))
        .sum()
}

fn main() {
    let rules = include_str!("../../inputs/day7.txt")
        .lines()
        .filter_map(parse_rule)
        .collect::<HashMap<&str, HashMap<_, _>>>();

    println!("{}", allowed_containers("shiny gold", &rules).len());
    println!("{}", count_contents("shiny gold", &rules));
}
