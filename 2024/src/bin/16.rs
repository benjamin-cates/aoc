use std::{cmp::Reverse, collections::{BTreeMap, HashMap, HashSet}};

use aoc24::{CharGrid, Direction, LazyGraph, Point, StaticGraph};
use priority_queue::PriorityQueue;

fn main() {
    let input: &str = include_str!("../data/16.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} ({:?})", part1(input), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} ({:?})", part2(input), now.elapsed());
}

// Finished in 1:03:51
fn part1(input: &str) -> usize {
    let grid = CharGrid::new(input);
    let mut list_of_edges: Vec<((Direction, Point), (Direction, Point), i64)> = vec![];
    for dir in [
        Direction::East,
        Direction::North,
        Direction::South,
        Direction::West,
    ] {
        for x in 0..grid.width() {
            for y in 0..grid.height() {
                let point: Point = (x, y).into();
                if grid.get(point) != Some('#') && grid.get(point + dir.step()) != Some('#') {
                    list_of_edges.push(((dir, (x, y).into()), (dir, point + dir.step()), 1));
                }
                if grid.get(point) != Some('#') {
                    list_of_edges.push(((dir, point), (dir.rotate_left(), point), 1000));
                    list_of_edges.push(((dir, point), (dir.rotate_right(), point), 1000));
                }
            }
        }
    }
    let map = StaticGraph::new().add_undirected_weighted_edges(list_of_edges.into_iter());
    let end = grid.find('E').unwrap();
    let path = map
        .dijkstras(
            &(Direction::East, grid.find('S').unwrap()),
            |(_dir, point)| *point == end,
        )
        .unwrap();
    let mut cur_direction = Direction::East;
    let mut score = 0;
    for (dir, _point) in path.into_iter().skip(1) {
        if dir != cur_direction {
            score += 1000;
            cur_direction = dir;
        } else {
            score += 1;
        }
    }
    score
}

/// Finished in 01:26:50
fn part2(input: &str) -> usize {
    let grid = CharGrid::new(input);
    let mut list_of_edges: Vec<((Direction, Point), (Direction, Point), i64)> = vec![];
    for dir in [
        Direction::East,
        Direction::North,
        Direction::South,
        Direction::West,
    ] {
        for x in 0..grid.width() {
            for y in 0..grid.height() {
                let point: Point = (x, y).into();
                if grid.get(point) != Some('#') && grid.get(point + dir.step()) != Some('#') {
                    list_of_edges.push(((dir, (x, y).into()), (dir, point + dir.step()), 1));
                }
                if grid.get(point) != Some('#') {
                    list_of_edges.push(((dir, point), (dir.rotate_left(), point), 1000));
                    list_of_edges.push(((dir, point), (dir.rotate_right(), point), 1000));
                }
            }
        }
    }
    let map = StaticGraph::new().add_undirected_weighted_edges(list_of_edges.into_iter());
    let end = grid.find('E').unwrap();
    let start_node = (Direction::East, grid.find('S').unwrap());
    let path = dijkstras_set(
            map.adjacency_lists,
            &map.nodes_set,
            &start_node,
            |(_dir, point)| *point == end,
        )
        .unwrap();
    path.len() + 1
}
type Node = (Direction, Point);
fn dijkstras_set<'a>(
    adjacency_lists: HashMap<(Direction, Point), BTreeMap<(Direction, Point), i64>>,
    nodes_set: &'a HashSet<(Direction, Point)>,
    source: &'a (Direction, Point),
    target: impl Fn(&'a (Direction, Point)) -> bool,
) -> Option<HashSet<&'a Point>> {
    let mut queue: PriorityQueue<&Node, Reverse<i64>> = PriorityQueue::new();
    let mut back_list: HashMap<&Node, HashSet<&Point>> = HashMap::new();
    for node in nodes_set.iter() {
        queue.push(node, Reverse(i64::MAX));
    }
    queue.change_priority(source, Reverse(0));
    while !queue.is_empty() {
        let (cur, cost) = queue.pop().unwrap();
        if cost.0 == i64::MAX {
            return None;
        }
        if target(cur) {
            return back_list.get(cur).cloned();
        }
        for (neighbor, dist) in adjacency_lists.get(cur).unwrap() {
            if let Some((_, neighbor_priority)) = queue.get(neighbor) {
                if neighbor_priority.0 == cost.0 + dist {
                    if back_list.get(cur).is_some() {
                        let to_insert = back_list.get(cur).unwrap().clone();
                        back_list.entry(neighbor).or_default().extend(to_insert);
                        back_list.entry(neighbor).or_default().insert(&cur.1);
                    }
                }
                if neighbor_priority.0 > cost.0 + dist {
                    queue.change_priority(neighbor, Reverse(cost.0 + dist));
                    let hash_set_b4 = match back_list.get(cur) {
                        Some(b4) => b4.clone(),
                        None => HashSet::new()
                    };
                    back_list.entry(neighbor).insert_entry(hash_set_b4);
                    back_list.entry(neighbor).or_default().insert(&cur.1);
                }

            }
        }
    }
    return None;
}
#[cfg(test)]
#[test]
fn test_example() {
    let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    assert_eq!(part1(input), 7036);
    assert_eq!(part2(input), 45);
    let input2 = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
    assert_eq!(part1(input2), 11048);
    assert_eq!(part2(input2), 64);
}
