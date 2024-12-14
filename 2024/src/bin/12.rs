use std::collections::{HashMap, HashSet};

use aoc24::{CharGrid, Direction, LazyGraph, Point};

fn main() {
    let input: &str = include_str!("../data/12.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} ({:?})", part1(input), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} ({:?})", part2(input), now.elapsed());
}

fn perimeter(group: &HashSet<Point>) -> usize {
    // Count a fence if a cell is adjacent to one not in the group
    use Direction::*;
    group
        .iter()
        .map(|v| {
            [North, South, East, West]
                .into_iter()
                .filter(|dir| group.get(&(*v + dir.step())).is_none())
                .count()
        })
        .sum::<usize>()
}
fn discount_perimeter(group: &HashSet<Point>) -> usize {
    use Direction::*;
    let dirs: [Direction; 4] = [North, South, East, West];
    // Basically we will count the open sides for each point in the group and cancel out open sides that are parallel
    let mut open_sides: HashMap<Point, [bool; 4]> = group
        .iter()
        .map(|v| {
            (
                *v,
                [North, South, East, West].map(|dir| group.get(&(*v + dir.step())).is_none()),
            )
        })
        .collect();
    for point in group {
        // Iterate over sides
        for i in 0..4 {
            if !open_sides.get(point).unwrap()[i] {
                continue;
            }
            // Rotate left does North => East, South => West, ...
            // Basically the direction parallel to the open edge on the left side
            // If there is as fence to the right, the one to the right will cancel this one out, so we don't need to iterate to the right
            let left = dirs[i].rotate_left();
            for j in 1.. {
                let sides = open_sides.get_mut(&(*point + left.step() * j));
                if sides.as_ref().map(|v| v[i]) != Some(true) {
                    break;
                }
                sides.unwrap()[i] = false;
            }
        }
    }
    // Count up all open sides
    open_sides
        .iter()
        .map(|(_point, open)| open.iter().filter(|v| **v).count())
        .sum::<usize>()
}

fn calculate_costs(input: &str, perimeter_counter: impl Fn(&HashSet<Point>) -> usize) -> usize {
    let grid = CharGrid::new(input);
    let grid_ref = &grid;
    let graph: LazyGraph<Point, _, _> = LazyGraph::from_fn(move |point: &Point| {
        let point = point.clone();
        use Direction::*;
        (&[North, South, East, West])
            .iter()
            .filter(|dir| grid_ref.get(point + dir.step()) == grid_ref.get(point))
            .collect::<Vec<_>>()
            .into_iter()
            .map(move |dir| (point + dir.step(), 1))
    });
    let width = grid.width();
    let height = grid.height();
    let mut answer = 0;
    let mut visited_set: HashSet<Point> = HashSet::new();
    for x in 0..width {
        for y in 0..height {
            if visited_set.contains(&(x, y).into()) {
                continue;
            }
            let group = graph.into_bfs_iter((x, y).into()).collect::<HashSet<_>>();
            for point in group.iter() {
                visited_set.insert(*point);
            }
            answer += perimeter_counter(&group) * group.len();
        }
    }
    answer
}

// Finished in 17:15
fn part1(input: &str) -> usize {
    calculate_costs(input, perimeter)
}

// Finished in 35:09
fn part2(input: &str) -> usize {
    calculate_costs(input, discount_perimeter)
}

#[cfg(test)]
#[test]
fn test_example() {
    let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    assert_eq!(part1(input), 1930);
    assert_eq!(part2(input), 1206);
}
