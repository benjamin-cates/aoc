use std::collections::HashMap;

use aoc24::{CharGrid, Direction, Point, StaticGraph};

fn main() {
    let input: &str = include_str!("../data/20.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} ({:?})", part1(input), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} ({:?})", part2(input), now.elapsed());
}

// Finished in 16:49
fn part1(input: &str) -> usize {
    let grid = CharGrid::new(input);
    let mut edges: Vec<(Point, Point)> = vec![];
    for point in grid.find_all('.') {
        for dir in Direction::iter_all() {
            if grid.get(point + dir.step()) != Some('#') {
                edges.push((point, point + dir.step()));
            }
        }
    }
    let graph = StaticGraph::new().add_undirected_edges(edges);
    let start = grid.find('S').unwrap();
    let path = graph.dijkstras(&start, |point| grid[*point] == 'E').unwrap();
    let path_positions = path.iter().enumerate().map(|(i, v)| (*v, i)).collect::<HashMap<Point, usize>>();
    let mut cuts = 0;
    for (val, pos) in path_positions.iter() {
        for dir in Direction::iter_all() {
            if grid.get(*val + dir.step()) != Some('#') {
                continue;
            }
            if let Some(new_pos) = path_positions.get(&(*val + dir.step() * 2)) {
                if new_pos < pos && pos - new_pos - 2 >= 100 {
                    cuts += 1;
                }
            }
        }
    }
    cuts
}

// Finished in 31:50
fn part2(input: &str) -> usize {
    let grid = CharGrid::new(input);
    let mut edges: Vec<(Point, Point)> = vec![];
    for point in grid.find_all('.') {
        for dir in Direction::iter_all() {
            if grid.get(point + dir.step()) != Some('#') {
                edges.push((point, point + dir.step()));
            }
        }
    }
    let graph = StaticGraph::new().add_undirected_edges(edges);
    let start = grid.find('S').unwrap();
    let path = graph.dijkstras(&start, |point| grid[*point] == 'E').unwrap();
    let path_positions = path.iter().enumerate().map(|(i, v)| (*v, i)).collect::<HashMap<Point, usize>>();
    let mut cuts = 0;
    for (val, pos) in path_positions.iter() {
        for delta_y in -20i32..=20i32 {
            for delta_x in (-20 + delta_y.abs())..=(20 - delta_y.abs()) {
                let delta = (delta_x, delta_y).into();
                if let Some(new_pos) = path_positions.get(&(*val + delta)) {
                    if new_pos < pos {
                        let saved = pos - new_pos - delta_y.abs() as usize - delta_x.abs() as usize;
                        if saved >= 100 {
                            cuts += 1;
                        }
                    }
                }
            }
        }
    }
    cuts
}

#[cfg(test)]
#[test]
fn test_example() {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    assert_eq!(part1(input), 0);
    assert_eq!(part2(input), 0);
}
