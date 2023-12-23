use aoc23::traverse::Dir::*;
use aoc23::traverse::*;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input: &str = include_str!("23.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

/// Get slope direction from character
fn dir_from_char(ch: char) -> Option<Dir> {
    match ch {
        'v' => Some(South),
        '^' => Some(North),
        '>' => Some(East),
        '<' => Some(West),
        _ => None,
    }
}

/// Returns true if a point is within grid bounds
fn point_in_bound(point: &Point, grid: &Vec<Vec<char>>) -> bool {
    point.x >= 0 && point.y >= 0 && point.y < grid.len() as i32 && point.x < grid[0].len() as i32
}

fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // Iterate through possible paths and find the maximum
    let mut to_traverse: VecDeque<(usize, Point)> = vec![(0, (1, 0).into())].into();
    let mut finished_paths: Vec<usize> = vec![];
    while let Some((mut dist, mut point)) = to_traverse.pop_front() {
        let mut prev = (0, 0).into();
        let mut changed = true;
        while changed {
            changed = false;
            for dir in [North, South, East, West] {
                let new_point = point + dir.step();
                if new_point == prev || !point_in_bound(&new_point, &grid) {
                    continue;
                }
                if point == (grid[0].len() as i32 - 2, grid.len() as i32 - 1).into() {
                    finished_paths.push(dist);
                    break;
                }
                let ch = grid[new_point.y as usize][new_point.x as usize];
                if ch == '.' {
                    prev = point;
                    point = new_point;
                    dist += 1;
                    changed = true;
                }
                let slope = dir_from_char(ch);
                if slope == Some(dir) {
                    to_traverse.push_back((dist + 2, new_point + dir.step()));
                }
            }
        }
    }
    *finished_paths.iter().max().unwrap()
}

/// Returns a list of forks in the graph (points that have 3 or 4 paths from them).
fn get_vertices(grid: &Vec<Vec<char>>) -> HashSet<Point> {
    let mut vertices: HashSet<Point> = HashSet::new();
    for y in 1..(grid.len() as i32 - 1) {
        for x in 1..(grid[0].len() as i32 - 1) {
            if grid[y as usize][x as usize] == '#' {
                continue;
            }
            let forks = [North, South, East, West]
                .iter()
                .filter(|dir| {
                    let looking_at: Point = Point::from((x, y)) + dir.step();
                    grid[looking_at.y as usize][looking_at.x as usize] != '#'
                })
                .count();
            if forks > 2 {
                vertices.insert((x, y).into());
            }
        }
    }
    vertices.insert((1, 0).into());
    vertices.insert((grid[0].len() as i32 - 2, grid.len() as i32 - 1).into());
    vertices
}

/// Returns a map of vertices to a list of the vertices it can go to
fn get_undirected_graph(
    grid: &Vec<Vec<char>>,
    vertices: &HashSet<Point>,
) -> HashMap<Point, Vec<(usize, Point)>> {
    let mut edges: HashMap<Point, Vec<(usize, Point)>> = HashMap::new();
    for vertex in vertices.iter() {
        edges.insert(*vertex, vec![]);
    }
    // For each vertex, test possible directions you could go
    for vertex in vertices.iter() {
        for dir in [North, South, East, West] {
            let mut point = *vertex + dir.step();
            let mut dist = 1;
            if !point_in_bound(&point, &grid) || grid[point.y as usize][point.x as usize] != '.' {
                continue;
            }
            // Follow the path until you get to another vertex
            let mut prev = *vertex;
            while vertices.get(&point).is_none() {
                for dir in [North, South, East, West] {
                    let new_point = point + dir.step();

                    if !point_in_bound(&new_point, &grid)
                        || new_point == prev
                        || grid[new_point.y as usize][new_point.x as usize] == '#'
                    {
                        continue;
                    }
                    prev = point;
                    point = new_point;
                    dist += 1;
                    break;
                }
            }
            if *vertex == point {
                continue;
            }
            edges.get_mut(&vertex).unwrap().push((dist, point));
        }
    }
    edges
}

/// Find the maximum possible path to get to the distination node from cur without going to the
/// visited set. Definitely not efficient lol, but it runs in under 10 seconds so we'll take it.
fn max_depth_first(
    cur: Point,
    destination: &Point,
    edges: &HashMap<Point, Vec<(usize, Point)>>,
    visited: &mut HashSet<Point>,
) -> usize {
    if *destination == cur {
        return 0;
    }
    visited.insert(cur);
    let mut max = 0;
    for (dist, point) in edges.get(&cur).unwrap().iter() {
        if visited.get(&point).is_some() {
            continue;
        }
        let total_dist = max_depth_first(*point, destination, edges, visited) + dist;
        if total_dist > max {
            max = total_dist;
        }
    }
    visited.remove(&cur);
    max
}

fn part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| match x {
                    '#' => '#',
                    _ => '.',
                })
                .collect()
        })
        .collect();
    let vertices = get_vertices(&grid);
    let edges = get_undirected_graph(&grid, &vertices);
    max_depth_first(
        (1, 0).into(),
        &(grid[0].len() as i32 - 2, grid.len() as i32 - 1).into(),
        &edges,
        &mut HashSet::new(),
    )
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    assert_eq!(part1(input), 94);
    assert_eq!(part2(input), 154);
    assert_eq!(part1(include_str!("23.txt")), 2174);
    assert_eq!(part2(include_str!("23.txt")), 6506);
}
