use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    str::FromStr,
};

type Position = (i32, i32);

#[derive(Clone, Default)]
struct Seat {
    occupied: bool,
    neighbors: HashSet<Position>,
}

impl Seat {
    fn new(occupied: bool) -> Self {
        Self {
            occupied,
            ..Self::default()
        }
    }
}

impl PartialEq<Seat> for Seat {
    fn eq(&self, other: &Seat) -> bool {
        self.occupied == other.occupied
    }
}

#[derive(Clone, PartialEq)]
struct SeatsLayout {
    rows: i32,
    columns: i32,
    seats: HashMap<Position, Seat>,
}

impl FromStr for SeatsLayout {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.lines().count();
        let columns = s.lines().next().ok_or(())?.chars().count();

        let seats = s
            .lines()
            .enumerate()
            .flat_map(|(x, line)| line.chars().enumerate().map(move |(y, c)| (x, y, c)))
            .filter_map(|(x, y, c)| {
                let pos = (x.try_into().unwrap(), y.try_into().unwrap());
                match c {
                    'L' => Some((pos, Seat::new(false))),
                    '#' => Some((pos, Seat::new(true))),
                    _ => None,
                }
            })
            .collect();

        Ok(Self {
            rows: rows.try_into().unwrap(),
            columns: columns.try_into().unwrap(),
            seats,
        })
    }
}

#[derive(Clone, Copy)]
enum Rule {
    Adjacency,
    FieldOfVision,
}

fn all_directions() -> impl Iterator<Item = Position> {
    (-1..=1)
        .flat_map(|x| (-1..=1).map(move |y| (x, y)))
        .filter(|(x, y)| *x != 0 || *y != 0)
}

impl SeatsLayout {
    fn occupancy(&self) -> usize {
        self.seats.values().filter(|seat| seat.occupied).count()
    }

    fn _locate_neighboring_seats(&mut self, rule: Rule) {
        let seats_positions = self.seats.keys().cloned().collect::<HashSet<_>>();
        let (max_x, max_y) = (self.rows, self.columns);

        for (&pos, seat) in &mut self.seats {
            seat.neighbors = match rule {
                Rule::Adjacency => {
                    let (x, y) = pos;
                    all_directions()
                        .map(move |(dx, dy)| (x + dx, y + dy))
                        .filter(|pos| seats_positions.contains(&pos))
                        .collect()
                }

                Rule::FieldOfVision => all_directions()
                    .filter_map(|(dx, dy)| {
                        std::iter::successors(Some(pos), |(x, y)| {
                            let new_x = x + dx;
                            let new_y = y + dy;

                            if (0..max_x).contains(&new_x) && (0..max_y).contains(&new_y) {
                                Some((new_x, new_y))
                            } else {
                                None
                            }
                        })
                        .skip(1)
                        .find(|pos| seats_positions.contains(&pos))
                    })
                    .collect(),
            };
        }
    }

    fn _step(&self, rule: Rule) -> Self {
        let new_layout = self
            .seats
            .iter()
            .map(|(&pos, seat)| {
                let mut new_seat = seat.clone();

                let occupied_neighbors = seat
                    .neighbors
                    .iter()
                    .filter(|pos| self.seats[pos].occupied)
                    .count();

                new_seat.occupied = match (rule, seat.occupied, occupied_neighbors) {
                    (_, false, 0) => true,
                    (Rule::FieldOfVision, true, n) if n >= 5 => false,
                    (Rule::Adjacency, true, n) if n >= 4 => false,
                    _ => seat.occupied,
                };

                (pos, new_seat)
            })
            .collect();

        Self {
            rows: self.rows,
            columns: self.columns,
            seats: new_layout,
        }
    }

    fn final_occupancy(mut self, rule: Rule) -> usize {
        fn fix<T: PartialEq>(first: T, f: impl Fn(&T) -> T) -> T {
            std::iter::successors(Some(first), |x| {
                let new = f(x);
                (new != *x).then(|| new)
            })
            .last()
            .unwrap()
        }

        self._locate_neighboring_seats(rule);
        fix(self, |layout| layout._step(rule)).occupancy()
    }
}

fn main() {
    let layout = include_str!("../../inputs/day11.txt")
        .parse::<SeatsLayout>()
        .unwrap();

    println!("{}", layout.clone().final_occupancy(Rule::Adjacency));
    println!("{}", layout.final_occupancy(Rule::FieldOfVision));
}
