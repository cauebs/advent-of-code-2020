use std::collections::BTreeSet;

fn parse_seat_id(s: &str) -> u32 {
    let (row_code, column_code) = s.split_at(7);

    let row_code = row_code.replace("F", "0").replace("B", "1");
    let column_code = column_code.replace("L", "0").replace("R", "1");

    let row = u32::from_str_radix(&row_code, 2).unwrap();
    let column = u32::from_str_radix(&column_code, 2).unwrap();

    row * 8 + column
}

fn main() {
    let seat_ids = include_str!("../../inputs/day5.txt")
        .lines()
        .map(parse_seat_id)
        .collect::<BTreeSet<u32>>();

    println!("{}", seat_ids.iter().next_back().unwrap());

    let mut adjacents = seat_ids.iter().zip(seat_ids.iter().skip(1));

    let missing = adjacents
        .find(|(&current, &next)| next != current + 1)
        .map(|(current, _)| current + 1)
        .unwrap();

    println!("{}", missing);
}
