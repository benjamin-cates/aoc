use std::collections::{BTreeSet, HashSet};

use aoc23::{CharGrid, Point};

fn main() {
    let input: &str = include_str!("../data/08.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} ({:?})", part1(input), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} ({:?})", part2(input), now.elapsed());
}

fn part1(input: &str) -> usize {
    let unique_chars = input
        .chars()
        .filter(|x| *x != '.' && !x.is_whitespace())
        .collect::<BTreeSet<_>>();
    let grid = CharGrid::new(input);
    let mut antinodes = HashSet::new();
    for freq in unique_chars.iter() {
        if *freq == '.' {
            continue;
        }
        let points = grid.find_all(*freq).collect::<Vec<Point>>();
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let dist: Point = points[i] + points[j] * -1;
                // Test one before points[i]
                if grid.get(points[i] + dist).is_some() {
                    antinodes.insert(points[i] + dist);
                }
                // Test one past points[j]
                if grid.get(points[j] + dist * -1).is_some() {
                    antinodes.insert(points[j] + dist * -1);
                }
            }
        }
    }
    antinodes.len()
}

fn part2(input: &str) -> usize {
    let unique_chars = input
        .chars()
        .filter(|x| *x != '.' && !x.is_whitespace())
        .collect::<BTreeSet<_>>();
    let grid = CharGrid::new(input);
    let mut antinodes = HashSet::new();
    for freq in unique_chars.iter() {
        let points = grid.find_all(*freq).collect::<Vec<Point>>();
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let dist: Point = points[i] + points[j] * -1;
                let mut away = 0;
                while grid.get(points[i] + dist * away).is_some() {
                    antinodes.insert(points[i] + dist * away);
                    away += 1;
                }
                let mut away = 0;
                while grid.get(points[j] + dist * -away).is_some() {
                    antinodes.insert(points[j] + dist * -away);
                    away += 1;
                }
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
#[test]
fn test_example() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    assert_eq!(part1(input), 14);
    assert_eq!(part2(input), 34);
}
