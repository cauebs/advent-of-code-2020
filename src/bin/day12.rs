use advent_of_code_2020::Vec2D;

fn part_one(instructions: &[(&str, f64)]) -> f64 {
    let mut heading = 0.0;
    let mut ship = Vec2D::default();

    for &(action, value) in instructions {
        match action {
            "N" => ship.y += value,
            "S" => ship.y -= value,
            "E" => ship.x += value,
            "W" => ship.x -= value,
            "L" => heading += value,
            "R" => heading -= value,
            "F" => {
                let (dy, dx) = heading.to_radians().sin_cos();
                let direction = Vec2D::new(dx, dy);
                ship += direction * value;
            }
            _ => panic!("invalid action"),
        }
    }

    ship.manhattan_distance()
}

fn part_two(instructions: &[(&str, f64)]) -> f64 {
    let mut waypoint = Vec2D::new(10.0, 1.0);
    let mut ship = Vec2D::default();

    for &(action, value) in instructions {
        match action {
            "N" => waypoint.y += value,
            "S" => waypoint.y -= value,
            "E" => waypoint.x += value,
            "W" => waypoint.x -= value,
            "L" => waypoint.rotate(value.to_radians()),
            "R" => waypoint.rotate(-value.to_radians()),
            "F" => {
                ship += waypoint * value;
            }
            _ => panic!("invalid action"),
        }
    }

    ship.manhattan_distance()
}

fn main() {
    let instructions = include_str!("../../inputs/day12.txt")
        .lines()
        .map(|line| {
            let (action, value) = line.split_at(1);
            value.parse().map(|v| (action, v))
        })
        .collect::<Result<Vec<(&str, f64)>, _>>()
        .unwrap();

    println!("{}", part_one(&instructions).round() as u32);
    println!("{}", part_two(&instructions).round() as u32);
}
