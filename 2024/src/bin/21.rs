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
    let actv: Point = (2, 0).into();
    if depth == 0 {
        return (x.abs() + y.abs()) as usize + 1;
    }
    let invalid_spot: Point = if top { (0, 3).into() } else { (0, 0).into() };
    if x == 0 && y == 0 {
        return 1;
    }
    let path = |from: Point, to: Point| find_fastest(from, to, depth - 1, false);
    let two_loop = |a: Point, b: Point| path(a, b) + path(b, a);
    if y == 0 && x < 0 {
        // Left
        return two_loop(actv, left) + x.abs() as usize - 1;
    }
    if y == 0 && x > 0 {
        // Rigth
        return two_loop(actv, right) + x.abs() as usize - 1;
    }
    if x == 0 && y < 0 {
        // Up
        return two_loop(actv, up) + y.abs() as usize - 1;
    }
    if x == 0 && y > 0 {
        // Down
        return two_loop(actv, down) + y.abs() as usize - 1;
    }
    let three_loop = |a: Point, b: Point, c: Point| path(a, b) + path(b, c) + path(c, a);
    let y_presses = y.abs() as usize - 1;
    let x_presses = x.abs() as usize - 1;
    if x < 0 && y < 0 {
        // Up and left
        return if start + (x, 0).into() == invalid_spot {
            three_loop(actv, up, left) + x_presses + y_presses
        } else {
            three_loop(actv, up, left).min(three_loop(actv, left, up)) + x_presses + y_presses
        };
    }
    if x < 0 && y > 0 {
        // Down and left
        return if start + (x, 0).into() == invalid_spot {
            three_loop(actv, down, left) + x_presses + y_presses
        } else {
            three_loop(actv, left, down).min(three_loop(actv, left, down)) + x_presses + y_presses
        };
    }
    if x > 0 && y < 0 {
        // Up and right
        return if start + (0, y).into() == invalid_spot {
            three_loop(actv, right, up) + x_presses + y_presses
        } else {
            three_loop(actv, right, up).min(three_loop(actv, up, right)) + x_presses + y_presses
        };
    }
    if x > 0 && y > 0 {
        // Down and right
        return if start + (0, y).into() == invalid_spot {
            three_loop(actv, right, down) + x_presses + y_presses
        } else {
            three_loop(actv, right, down).min(three_loop(actv, down, right)) + x_presses + y_presses
        };
    }
    unreachable!()
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
