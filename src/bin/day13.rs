fn apply_chinese_remainder_theorem(constants: &[i64], divisors: &[i64]) -> i64 {
    // assumes divisors are coprime

    let lcm = divisors.iter().product::<i64>();

    let solution = divisors
        .iter()
        .zip(constants.into_iter())
        .map(|(&n, a)| {
            // each term of the solution is divisible (0 mod) by any n other than this
            let term = lcm / n;

            // multiply the term until it is congruent to `a mod n`
            let term = (1..)
                .map(|factor| (term * factor))
                .find(|term| term.rem_euclid(n) == a.rem_euclid(n))
                .unwrap();

            // term is still `0 mod n` for all other n, but `a mod n` for this n

            term
        })
        .sum::<i64>();

    solution % lcm
}

fn main() {
    let mut lines = include_str!("../../inputs/day13.txt").lines();

    let current_time = lines.next().unwrap().parse::<u32>().unwrap();

    let bus_ids = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| match s {
            "x" => None,
            id => Some(id.parse().unwrap()),
        })
        .collect::<Vec<Option<u32>>>();

    let (next_bus, wait) = bus_ids
        .iter()
        .filter_map(|&id| id)
        .map(|id| {
            let period = id;
            let cycles = (f64::from(current_time) / f64::from(period)).ceil() as u32;
            let next_arrival = cycles * period;
            let wait = next_arrival - current_time;
            (id, wait)
        })
        .min_by_key(|&(_id, wait)| wait)
        .unwrap();

    println!("{}", next_bus * wait);

    let (periods, offsets): (Vec<_>, Vec<_>) = bus_ids
        .iter()
        .enumerate()
        .filter_map(|(i, id)| {
            let offset = -(i as i64);
            id.map(|period| (period as i64, offset))
        })
        .unzip();

    let solution = apply_chinese_remainder_theorem(&offsets, &periods);

    assert_eq!(solution, 702970661767766);
    println!("{}", solution);
}
