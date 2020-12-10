use std::collections::BTreeMap;

fn main() {
    let input = include_str!("../../inputs/day3.txt");

    let tree_map: BTreeMap<_, _> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x, y), c == '#'))
        })
        .collect();

    let &(max_x, max_y) = tree_map.keys().next_back().unwrap();

    let trees_across_slope = |(right, down)| {
        let horizontal = (0..=max_x).cycle().step_by(right);
        let vertical = (0..=max_y).step_by(down);

        horizontal
            .zip(vertical)
            .filter(|(x, y)| tree_map[&(*x, *y)])
            .count()
    };

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    println!(
        "{}",
        slopes
            .into_iter()
            .map(trees_across_slope)
            .product::<usize>()
    );
}
