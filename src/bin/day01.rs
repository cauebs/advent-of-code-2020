use std::collections::HashSet;

const TARGET: u32 = 2020;

fn main() {
    let lines = include_str!("../../inputs/day1.txt").lines();

    let (low_values, high_values): (HashSet<_>, HashSet<_>) = lines
        .map(|line| line.parse::<u32>().expect("line must contain digits only"))
        .partition(|v| *v <= TARGET / 2);

    // part 1
    for lo in &low_values {
        if let Some(hi) = TARGET.checked_sub(*lo) {
            if high_values.contains(&hi) {
                println!("{}", lo * hi);
                break;
            }
        }
    }

    // part 2
    'outer: for (i, lo1) in low_values.iter().enumerate() {
        for lo2 in low_values.iter().skip(i) {
            if let Some(hi) = TARGET.checked_sub(*lo1 + *lo2) {
                if high_values.contains(&hi) {
                    println!("{}", lo1 * lo2 * hi);
                    break 'outer;
                }
            }
        }
    }
}
