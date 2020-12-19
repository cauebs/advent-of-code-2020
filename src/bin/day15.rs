use std::collections::HashMap;

fn dumb_quadratic(starting: &[u32], iterations: usize) -> u32 {
    let mut numbers = Vec::new();
    numbers.extend_from_slice(starting);

    for _ in starting.len()..iterations {
        let previous = numbers.last().unwrap();

        let next = numbers
            .iter()
            .rev()
            .skip(1)
            .position(|n| n == previous)
            .map(|i| i + 1)
            .unwrap_or(0) as u32;

        numbers.push(next);
    }

    *numbers.last().unwrap()
}

fn clever_linear(starting: &[u32], iterations: usize) -> u32 {
    let mut last_occurrence = HashMap::new();

    let all_but_last = &starting[0..starting.len() - 1];
    for (i, &n) in all_but_last.iter().enumerate() {
        last_occurrence.insert(n, i);
    }

    let mut previous = *starting.last().unwrap();

    for i in starting.len()..iterations {
        let next = last_occurrence
            .get(&previous)
            .map(|l| i - l - 1)
            .unwrap_or(0) as u32;

        last_occurrence.insert(previous, i - 1);
        previous = next;
    }

    previous
}

fn main() {
    let numbers = include_str!("../../inputs/day15.txt")
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<u32>, _>>()
        .unwrap();

    println!("{}", dumb_quadratic(&numbers, 2020));
    println!("{}", clever_linear(&numbers, 30_000_000));
}
