use std::collections::{HashSet, VecDeque};

use aoc24::{CharGrid, LazyGraph, Point};

fn main() {
    let input: &str = include_str!("../data/10.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} ({:?})", other_part1(input), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} ({:?})", part2(input), now.elapsed());
}

// finished in 15:01
fn part1(input: &str) -> usize {
    let grid = CharGrid::new(input);
    let trailheads = grid.find_all('0').collect::<Vec<_>>();
    let mut answer = 0;
    for trailhead in trailheads {
        let mut visited: HashSet<Point> = HashSet::new();
        let mut to_visit: VecDeque<Point> = VecDeque::new();
        to_visit.push_back(trailhead);
        while to_visit.len() != 0 {
            let cur = to_visit.pop_front().unwrap();
            use aoc24::Direction::*;
            for dir in [North, South, East, West] {
                let place = cur + dir.step();
                if visited.get(&place).is_some() {
                    continue;
                }
                if let Some(val) = grid.get(place) {
                    if (val as u8) == (grid.get_unwrap(cur) as u8) + 1 {
                        visited.insert(place);
                        if val == '9' {
                            answer += 1;
                        } else {
                            to_visit.push_back(place);
                        }
                    }
                }
            }
        }
    }
    answer
}

use aoc24::Direction::*;
fn other_part1(input: &str) -> usize {
    let grid = CharGrid::new(input);
    // LazyGraph function returns the outgoing edges for any given node
    let graph = LazyGraph::<Point, _, _>::from_fn(|node| {
        // Get expected height for next step
        let expected = (grid.get(*node).unwrap() as u8 + 1) as char;
        let grid = &grid;
        let node = node.clone();
        // Iterate over points that we could hike up
        ([North, South, East, West])
            .into_iter()
            .map(move |dir| node + dir.step())
            .filter(move |point| grid.get(*point) == Some(expected))
            .map(|point| (point, 1))
    });
    // For all trailheads, count the paths to 9 and sum them up
    grid.find_all('0')
        .map(|trailhead| {
            graph
                .into_bfs_iter(trailhead)
                .filter(|point| grid.get(*point) == Some('9'))
                .count()
        })
        .sum::<usize>()
}

// Finished in 15:50
fn part2(input: &str) -> usize {
    let grid = CharGrid::new(input);
    let trailheads = grid.find_all('0').collect::<Vec<_>>();
    let mut answer = 0;
    for trailhead in trailheads {
        let mut to_visit: VecDeque<Point> = VecDeque::new();
        to_visit.push_back(trailhead);
        while to_visit.len() != 0 {
            let cur = to_visit.pop_front().unwrap();
            use aoc24::Direction::*;
            for dir in [North, South, East, West] {
                let place = cur + dir.step();
                if let Some(val) = grid.get(place) {
                    if (val as u8) == (grid.get(cur).unwrap() as u8) + 1 {
                        if val == '9' {
                            answer += 1;
                        } else {
                            to_visit.push_back(place);
                        }
                    }
                }
            }
        }
    }
    answer
}

#[cfg(test)]
#[test]
fn test_example() {
    let input = "0123
1234
8765
9876";
    assert_eq!(part1(input), 1);
    let input = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
    assert_eq!(part1(input), 2);
    let input = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
    assert_eq!(part1(input), 4);
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    assert_eq!(part1(input), 36);
    assert_eq!(part2(input), 0);
}
