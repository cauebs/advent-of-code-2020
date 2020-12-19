#![feature(array_windows, entry_insert, is_sorted)]

use std::collections::HashMap;

fn full_sequence_deltas(adapters: &[u32]) -> HashMap<u32, u32> {
    assert!(adapters.is_sorted());

    let mut deltas = HashMap::new();

    for [a, b] in adapters.array_windows() {
        *deltas.entry(b - a).or_insert(0) += 1;
    }

    deltas
}

fn possible_sequences_graph(adapters: &[u32]) -> HashMap<u32, u64> {
    assert!(adapters.is_sorted());

    let mut sequences_up_to = adapters
        .iter()
        .map(|joltage| (*joltage, 0))
        .collect::<HashMap<_, _>>();

    // start at the outlet, with only one possible path
    sequences_up_to.entry(0).insert(1);

    for adapter in adapters {
        let sequences_up_to_this = sequences_up_to[adapter];

        if sequences_up_to_this == 0 {
            // the input guarantees that all adapters can be used
            unreachable!();
        }

        for delta in 1..=3 {
            sequences_up_to
                .entry(adapter + delta)
                .and_modify(|n| *n += sequences_up_to_this);
        }
    }

    sequences_up_to
}

fn main() {
    let mut adapters = include_str!("../../inputs/day10.txt")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<u32>>();

    // outlet
    adapters.push(0);

    let device_builtin = adapters.iter().max().unwrap() + 3;
    adapters.push(device_builtin);

    adapters.sort_unstable();

    // part 1
    let deltas = full_sequence_deltas(&adapters);
    println!("{}", deltas[&1] * deltas[&3]);

    // part 2
    println!("{}", possible_sequences_graph(&adapters)[&device_builtin]);
}
