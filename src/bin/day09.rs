fn is_sum_of_two(target: u32, possible_values: &[u32]) -> bool {
    for (i, a) in possible_values.iter().enumerate() {
        for b in possible_values.iter().skip(i + 1) {
            if a + b == target {
                return true;
            }
        }
    }

    false
}

fn find_contiguous_sum(target: u32, slice: &[u32]) -> Option<&[u32]> {
    for (first_pos, &a) in slice.iter().enumerate() {
        let mut total = a;

        for (last_pos, &b) in slice.iter().enumerate().skip(first_pos + 1) {
            total += b;

            if total > target {
                break;
            } else if total == target {
                return Some(&slice[first_pos..=last_pos]);
            }
        }
    }

    None
}

fn main() {
    let stream = include_str!("../../inputs/day09.txt")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<u32>>();

    let invalid = stream
        .windows(25 + 1)
        .find(|slice| !is_sum_of_two(slice[25], &slice[0..25]))
        .unwrap()[25];

    println!("{}", invalid);

    let slice = find_contiguous_sum(invalid, &stream).unwrap();
    let min = slice.iter().min().unwrap();
    let max = slice.iter().max().unwrap();

    println!("{}", min + max);
}
