use aoc24::{CharGrid, Point};
use cached::proc_macro::cached;

fn main() {
    let input: &str = include_str!("../data/21.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} ({:?})", part1(input), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} ({:?})", part2(input), now.elapsed());
}

#[cached]
fn find_fastest(start: Point, end: Point, depth: usize, top: bool) -> usize {
    let x = end.x - start.x;
    let y = end.y - start.y;
    let left: Point = (0, 1).into();
    let right: Point = (2, 1).into();
    let up: Point = (1, 0).into();
    let down: Point = (1, 1).into();
    let activate: Point = (2, 0).into();
    if depth == 0 {
        return (x.abs() + y.abs()) as usize + 1;
    }
    let invalid_spot: Point = if top { (0, 3).into() } else { (0, 0).into() };
    let out = if x < 0 {
        if y < 0 {
            // Up and left
            let path_1 = find_fastest(activate, up, depth - 1, false)
                + find_fastest(up, left, depth - 1, false)
                + find_fastest(left, activate, depth - 1, false)
                + (y.abs() as usize - 1)
                + (x.abs() as usize - 1);
            if start.x + x == invalid_spot.x && start.y == invalid_spot.y {
                path_1
            } else {
                let path_2 = find_fastest(activate, left, depth - 1, false)
                    + find_fastest(left, up, depth - 1, false)
                    + find_fastest(up, activate, depth - 1, false)
                    + (y.abs() as usize - 1)
                    + (x.abs() as usize - 1);
                path_1.min(path_2)
            }
        } else if y == 0 {
            // Left
            let out = find_fastest(activate, left, depth - 1, false)
                + find_fastest(left, activate, depth - 1, false)
                + (x.abs() as usize - 1);
            out
        } else {
            // Down and left
            let path_1 = find_fastest(activate, down, depth - 1, false)
                + find_fastest(down, left, depth - 1, false)
                + find_fastest(left, activate, depth - 1, false)
                + (y.abs() as usize - 1)
                + (x.abs() as usize - 1);
            if start.x + x == invalid_spot.x && start.y == invalid_spot.y {
                path_1
            } else {
                let path_2 = find_fastest(activate, left, depth - 1, false)
                    + (y.abs() as usize - 1)
                    + find_fastest(left, down, depth - 1, false)
                    + (x.abs() as usize - 1)
                    + find_fastest(down, activate, depth - 1, false);
                path_1.min(path_2)
            }
        }
    } else if x == 0 {
        if y < 0 {
            // Up
            find_fastest(activate, up, depth - 1, false)
                + (y.abs() as usize - 1)
                + find_fastest(up, activate, depth - 1, false)
        } else if y == 0 {
            1
        } else {
            // Down
            find_fastest(activate, down, depth - 1, false)
                + (y.abs() as usize - 1)
                + find_fastest(down, activate, depth - 1, false)
        }
    } else {
        if y < 0 {
            // Up and right
            let path_2 = find_fastest(activate, right, depth - 1, false)
                + (y.abs() as usize - 1)
                + find_fastest(right, up, depth - 1, false)
                + (x.abs() as usize - 1)
                + find_fastest(up, activate, depth - 1, false);
            if start.x == invalid_spot.x && start.y + y == invalid_spot.y {
                path_2
            } else {
                let path_1 = find_fastest(activate, up, depth - 1, false)
                    + (y.abs() as usize - 1)
                    + find_fastest(up, right, depth - 1, false)
                    + (x.abs() as usize - 1)
                    + find_fastest(right, activate, depth - 1, false);
                path_1.min(path_2)
            }
        } else if y == 0 {
            // Right
            find_fastest(activate, right, depth - 1, false)
                + (x.abs() as usize - 1)
                + find_fastest(right, activate, depth - 1, false)
        } else {
            // Down and right
            let path_2 = find_fastest(activate, right, depth - 1, false)
                + (y.abs() as usize - 1)
                + find_fastest(right, down, depth - 1, false)
                + (x.abs() as usize - 1)
                + find_fastest(down, activate, depth - 1, false);
            if start.y + y == invalid_spot.y && start.x == invalid_spot.x {
                path_2
            } else {
                let path_1 = find_fastest(activate, down, depth - 1, false)
                    + (y.abs() as usize - 1)
                    + find_fastest(down, right, depth - 1, false)
                    + (x.abs() as usize - 1)
                    + find_fastest(right, activate, depth - 1, false);
                path_1.min(path_2)
            }
        }
    };
    out
}

// Finished in 03:45:23
fn part1(input: &str) -> usize {
    let keypad = CharGrid::new("789\n456\n123\n 0A");
    let mut out = 0;
    for line in input.lines() {
        let points: Vec<Point> = line.chars().map(|v| keypad.find(v).unwrap()).collect();
        let mut cur: Point = (2, 3).into();
        let mut sum = 0;
        for point in points {
            sum += find_fastest(cur, point, 2, true);
            cur = point;
        }
        out += line[0..3].parse::<usize>().unwrap() * sum;
    }
    out
}

// Finished next day after 14:44:05
fn part2(input: &str) -> usize {
    let keypad = CharGrid::new("789\n456\n123\n 0A");
    let mut out = 0;
    for line in input.lines() {
        let points: Vec<Point> = line.chars().map(|v| keypad.find(v).unwrap()).collect();
        let mut cur = (2, 3).into();
        let mut sum = 0;
        for point in points {
            sum += find_fastest(cur, point, 25, true);
            cur = point;
        }
        out += line[0..3].parse::<usize>().unwrap() * sum;
    }
    out
}

#[cfg(test)]
#[test]
fn test_example() {
    let input = "029A
980A
179A
456A
379A";
    assert_eq!(part1(input), 126384);
    assert_eq!(part2(input), 154115708116294);
}
