use std::collections::HashSet;

use aoc23::traverse::Dir::*;
use aoc23::traverse::*;
fn main() {
    let input: &str = include_str!("../data/21.txt");
    println!("Answer to part1: {}", part1(input, 64));
    println!("Answer to part2: {}", part2(input, 26501365));
}

/// Return the number of filled spots in a grid given a certain list of enty points and
fn fill_grid(grid: &Vec<Vec<char>>, entry: Point, time: usize) -> usize {
    // List of points they could be on
    let mut points: HashSet<Point> = HashSet::new();
    points.insert(entry);
    // List of points they can step to on the next step
    let mut next_points: HashSet<Point> = HashSet::new();
    // History for each step so we can extrapolate answers
    let mut hist: Vec<usize> = vec![];
    for i in (0..time).rev() {
        for point in points {
            for dir in [North, South, East, West] {
                let next = point + dir.step();
                if next.y < 0
                    || next.x < 0
                    || next.y >= grid.len() as i32
                    || next.x >= grid[next.y as usize].len() as i32
                {
                    continue;
                }
                if grid[next.y as usize][next.x as usize] != '#' {
                    next_points.insert(next);
                }
            }
        }
        points = next_points;
        next_points = HashSet::new();
        hist.push(points.len());
        // If it oscillates with size 2, the grid will keep oscillating, so determine the output
        if hist.len() > 2 && hist[hist.len() - 3] == hist[hist.len() - 1] {
            if i % 2 == 0 {
                return hist[hist.len() - 3];
            } else {
                return hist[hist.len() - 2];
            }
        }
    }
    return points.len();
}

/// Returns the starting point of the garden grid
fn get_start(grid: &Vec<Vec<char>>) -> Point {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                return (x as i32, y as i32).into();
            }
        }
    }
    panic!("Cannot find start position");
}

fn part1(input: &str, steps: usize) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let start: Point = get_start(&grid);
    fill_grid(&grid, start, steps)
}

/// This function assumes the grid is a square and has an odd side length
fn part2(input: &str, count: usize) -> usize {
    // Count is the number of steps left
    // It is decremented after each step
    // when it reaches zero, stop moving
    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let start = get_start(&grid);
    let width = grid.len() as i32;
    // The number of blocks in any direction that you can fully cover within count steps
    let manhattan = (count - 1) / width as usize;
    // Center garden
    let mut total = fill_grid(&grid, start, count);
    // Interior gardengs
    let even_garden = fill_grid(&grid, start, count);
    let odd_garden = fill_grid(&grid, start, count - 1);
    for i in 1..(manhattan as usize) {
        if i % 2 == 0 {
            total += even_garden * 4 * i;
        } else {
            total += odd_garden * 4 * i;
        }
    }
    let corners = [
        (0, 0),
        (0, width - 1),
        (width - 1, 0),
        (width - 1, width - 1),
    ];
    // Far corners
    if manhattan != 0 && count > manhattan * width as usize {
        let steps_left = count - manhattan * width as usize - 1;
        for corner in corners {
            total += fill_grid(&grid, corner.into(), steps_left) * manhattan;
        }
    }
    // Closer corners
    if manhattan > 1 {
        let steps_left = count - manhattan * width as usize - 1 + width as usize;
        for corner in corners {
            total += fill_grid(&grid, corner.into(), steps_left) * (manhattan - 1);
        }
    }
    let edges = [
        (0, width / 2),
        (width / 2, 0),
        (width - 1, width / 2),
        (width / 2, width - 1),
    ];
    // Non full tips of the diamond
    if manhattan != 0 || count > width as usize / 2 {
        let steps_left = (count - ((width as usize) / 2 + 1)) % width as usize;
        for edge in edges {
            total += fill_grid(&grid, edge.into(), steps_left);
        }
    }
    // Small peeking parts of the tips of the diamond
    if manhattan != 0 && count >= manhattan * width as usize + (width as usize / 2 + 1) {
        let steps_left = (count - ((width as usize) / 2 + 1)) % width as usize + width as usize;
        for edge in edges {
            total += fill_grid(&grid, edge.into(), steps_left);
        }
    }
    return total;
}

#[cfg(test)]
#[test]
fn test_solution() {
    // Part 1 test that was given
    let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    assert_eq!(part1(input, 6), 16);
    // Tests with open field
    let input = ".....
.....
..S..
.....
.....";
    for i in 1..1000 {
        assert_eq!(part2(input, i), (i + 1) * (i + 1));
    }
    let input = ".......
.......
.......
...S...
.......
.......
.......";
    for i in 1..1000 {
        assert_eq!(part2(input, i), (i + 1) * (i + 1));
    }
}
