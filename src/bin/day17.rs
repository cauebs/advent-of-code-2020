use std::{collections::HashSet, hash::Hash};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos3D(i32, i32, i32);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos4D(i32, i32, i32, i32);

impl Pos3D {
    fn to_4d(&self) -> Pos4D {
        let Self(x, y, z) = *self;
        Pos4D(x, y, z, 0)
    }
}

trait Position: Eq + Hash {
    fn neighbors(&self) -> Box<dyn Iterator<Item = Self>>;
}

impl Position for Pos3D {
    fn neighbors(&self) -> Box<dyn Iterator<Item = Self>> {
        let Self(x, y, z) = *self;
        Box::new(
            (-1..=1)
                .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                .flat_map(|(dx, dy)| (-1..=1).map(move |dz| (dx, dy, dz)))
                .filter(|&delta| delta != (0, 0, 0))
                .map(move |(dx, dy, dz)| Self(x + dx, y + dy, z + dz)),
        )
    }
}

impl Position for Pos4D {
    fn neighbors(&self) -> Box<dyn Iterator<Item = Self>> {
        let Self(x, y, z, w) = *self;
        Box::new(
            (-1..=1)
                .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                .flat_map(|(dx, dy)| (-1..=1).map(move |dz| (dx, dy, dz)))
                .flat_map(|(dx, dy, dz)| (-1..=1).map(move |dw| (dx, dy, dz, dw)))
                .filter(|&delta| delta != (0, 0, 0, 0))
                .map(move |(dx, dy, dz, dw)| Self(x + dx, y + dy, z + dz, w + dw)),
        )
    }
}

fn stays_active<P: Position>(cubes: &HashSet<P>, cube: &P) -> bool {
    let active_neighbors = cube
        .neighbors()
        .filter(|neighbor| cubes.contains(&neighbor))
        .count();
    (2..=3).contains(&active_neighbors)
}

fn activates<P: Position>(cubes: &HashSet<P>, pos: &P) -> bool {
    let active_neighbors = pos
        .neighbors()
        .filter(|neighbor| cubes.contains(&neighbor))
        .count();
    active_neighbors == 3
}

fn simulate<P: Position + Clone>(starting_cubes: &HashSet<P>) -> impl Iterator<Item = HashSet<P>> {
    std::iter::successors(Some(starting_cubes.clone()), |cubes| {
        let mut new_cubes = HashSet::new();

        new_cubes.extend(
            cubes
                .iter()
                .filter(|pos| stays_active(&cubes, pos))
                .cloned(),
        );

        let might_activate = cubes.iter().flat_map(Position::neighbors);
        new_cubes.extend(might_activate.filter(|pos| activates(&cubes, pos)));

        Some(new_cubes)
    })
}

fn main() {
    let input = include_str!("../../inputs/day17.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == '#').then(|| Pos3D(x as i32, y as i32, 0)))
        });

    let cubes_3d = input.collect();
    println!("{}", simulate(&cubes_3d).nth(6).unwrap().len());

    let cubes_4d = cubes_3d.iter().map(Pos3D::to_4d).collect();
    println!("{}", simulate(&cubes_4d).nth(6).unwrap().len());
}
