use aoc23::traverse::Dir::*;
use aoc23::traverse::*;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
fn main() {
    let input: &str = include_str!("18.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

/// Returns list of points from trench_map and also returns bounding box
fn parse_path(input: &str) -> (Vec<Point>, i32, i32, i32, i32) {
    let mut point: Point = (0, 0).into();
    let mut path: Vec<Point> = vec![];
    let mut xmin = 0;
    let mut ymin = 0;
    let mut xmax = 0;
    let mut ymax = 0;
    for command in input.lines() {
        let dir = match command.split(" ").nth(0).unwrap() {
            "R" => East,
            "D" => South,
            "L" => West,
            "U" => North,
            _ => panic!("Unknown direction"),
        };
        let num = command.split(" ").nth(1).unwrap().parse().unwrap();
        for _ in 0..num {
            path.push(point);
            point = point + dir.step();
            xmax = point.x.max(xmax);
            ymax = point.y.max(ymax);
            xmin = point.x.min(xmin);
            ymin = point.y.min(ymin);
        }
    }
    (path, xmax, ymax, xmin, ymin)
}

/// Caluclates the size of the lagoon given a trench map and a mapping from coords to cell size
fn calculate_lagoon_size(
    trench_map: &str,
    x_weights: &HashMap<i32, usize>,
    y_weights: &HashMap<i32, usize>,
) -> usize {
    let (path, xmax, ymax, xmin, ymin) = parse_path(trench_map);
    let path_set: HashSet<Point> = path.iter().copied().collect();
    let mut lagoon_size = 0;
    // Iterate over rows
    for y in ymin..=ymax {
        let mut is_in_trench = false;
        let mut prev_dir = None;
        // Iterate over cells in a row
        for x in xmin..=xmax {
            let cell_size = match x_weights.get(&x) {
                Some(val) => *val,
                None => 1,
            } * match y_weights.get(&y) {
                Some(val) => *val,
                None => 1,
            };
            // If (x,y) is on trench path
            if path_set.get(&(x, y).into()).is_some() {
                lagoon_size += cell_size;
                let above = path_set.get(&(x, y - 1).into()).is_some();
                let below = path_set.get(&(x, y + 1).into()).is_some();
                // Solid vertical lines are a trench boundary
                if above && below {
                    is_in_trench = !is_in_trench;
                    continue;
                }
                if prev_dir.is_some() {
                    // Changes is_in_trench if the stretch of cells is not a kiss
                    // Basically if prev_dir and current dir are the same, the path just kissed
                    // this row and did not cross it, so don't count it
                    if (above && prev_dir == Some(South)) || (below && prev_dir == Some(North)) {
                        is_in_trench = !is_in_trench;
                    }
                    continue;
                } else {
                    if above {
                        prev_dir = Some(North);
                    } else if below {
                        prev_dir = Some(South);
                    } else {
                        panic!("Trench is not a continuous loop");
                    }
                }
            } else {
                if is_in_trench {
                    lagoon_size += cell_size;
                }
                prev_dir = None;
            }
        }
    }
    lagoon_size
}

fn part1(input: &str) -> usize {
    calculate_lagoon_size(input, &HashMap::new(), &HashMap::new())
}

/// Parse the color so that the first 5 digits is a hexadecimal length
/// and the last digit is the direction
fn parse_colored_path(input: &str) -> Vec<(Dir, usize)> {
    input
        .lines()
        .map(|line| {
            let col = line.split("#").nth(1).unwrap().split(")").nth(0).unwrap();
            let len = usize::from_str_radix(&col[0..5], 16).unwrap();
            let dir = match col.chars().nth(5).unwrap() {
                '0' => East,
                '1' => South,
                '2' => West,
                '3' => North,
                ch => panic!("Unexpected char {}", ch),
            };
            (dir, len)
        })
        .collect()
}

fn part2(input: &str) -> usize {
    let path = parse_colored_path(input);
    // Calculate where the x axis and y axis are split by corners of the trench
    let mut x_splits: BTreeSet<i32> = BTreeSet::from([0]);
    let mut y_splits: BTreeSet<i32> = BTreeSet::from([0]);
    let mut point: Point = (0, 0).into();
    for (dir, len) in path.iter() {
        point = point + dir.step() * (*len as i32);
        x_splits.insert(point.x);
        y_splits.insert(point.y);
    }
    let x_splits: Vec<i32> = x_splits.into_iter().collect();
    let y_splits: Vec<i32> = y_splits.into_iter().collect();
    // Generate commands to traverse a scaled down trench with variable width cells
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut x_idx = x_splits.iter().position(|x| *x == 0).unwrap() as i32;
    let mut y_idx = y_splits.iter().position(|y| *y == 0).unwrap() as i32;
    let command = path
        .iter()
        .map(|(dir, len)| {
            let diff = match *dir {
                North | South => {
                    let new_y = y_pos + if *dir == North { -1 } else { 1 } * *len as i32;
                    let new_y_idx = y_splits.iter().position(|y| *y == new_y).unwrap() as i32;
                    let diff_y = y_idx.abs_diff(new_y_idx);
                    y_pos = new_y;
                    y_idx = new_y_idx;
                    diff_y
                }
                East | West => {
                    let new_x = x_pos + if *dir == West { -1 } else { 1 } * *len as i32;
                    let new_x_idx = x_splits.iter().position(|x| *x == new_x).unwrap() as i32;
                    let diff_x = x_idx.abs_diff(new_x_idx);
                    x_pos = new_x;
                    x_idx = new_x_idx;
                    diff_x
                }
            };
            format!(
                "{} {} ",
                match *dir {
                    North => "U",
                    South => "D",
                    East => "R",
                    West => "L",
                },
                diff * 2
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    // Find the width of each cell on the x and y axes
    let x_map = get_width_map(&x_splits);
    let y_map = get_width_map(&y_splits);
    calculate_lagoon_size(command.as_str(), &x_map, &y_map)
}

fn get_width_map(splits: &Vec<i32>) -> HashMap<i32, usize> {
    // Find the width of each cell on the y axis
    let mut map = HashMap::new();
    let zero_pos = splits.iter().position(|x| *x == 0).unwrap() as i32;
    for i in 0..(splits.len() - 1) {
        map.insert(
            (i as i32 - zero_pos) * 2 + 1,
            (splits[i + 1] - splits[i] - 1) as usize,
        );
    }
    map
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    assert_eq!(part1(include_str!("18.txt")), 36807);
    assert_eq!(part1(input), 62);
    assert_eq!(part2(include_str!("18.txt")), 48797603984357);
    assert_eq!(part2(input), 952408144115);
}
