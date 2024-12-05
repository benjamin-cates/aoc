use std::collections::HashSet;
fn main() {
    let input: &str = include_str!("../data/10.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

#[derive(PartialEq, Debug, Eq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn direct(dir: Direction, ch: char) -> Option<Direction> {
    match (ch, dir) {
        ('7', North) => Some(West),
        ('7', East) => Some(South),
        ('J', East) => Some(North),
        ('J', South) => Some(West),
        ('F', North) => Some(East),
        ('F', West) => Some(South),
        ('L', South) => Some(East),
        ('L', West) => Some(North),
        ('|', North) => Some(North),
        ('|', South) => Some(South),
        ('-', West) => Some(West),
        ('-', East) => Some(East),
        _ => None,
    }
}
use Direction::*;

fn traverse(
    input: &Vec<Vec<char>>,
    mut dir: Direction,
    mut pos: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut out: Vec<(usize, usize)> = vec![pos];
    loop {
        if dir == North {
            if pos.1 == 0 {
                return None;
            }
            pos.1 -= 1;
        } else if dir == South {
            if pos.1 == input.len() - 1 {
                return None;
            }
            pos.1 += 1;
        } else if dir == West {
            if pos.0 == 0 {
                return None;
            }
            pos.0 -= 1;
        } else if dir == East {
            if pos.0 == input[0].len() - 1 {
                return None;
            }
            pos.0 += 1
        }
        if input[pos.1][pos.0] == 'S' {
            return Some(out);
        }
        out.push(pos);
        dir = direct(dir, input[pos.1][pos.0])?;
    }
}

fn get_traversal(input: &str) -> Vec<(usize, usize)> {
    let input: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let start_y = input
        .iter()
        .position(|x| x.iter().position(|x| *x == 'S').is_some())
        .unwrap();
    let start_x = input[start_y].iter().position(|x| *x == 'S').unwrap();
    let start = (start_x, start_y);
    for dir in [North, South, East, West] {
        match traverse(&input, dir, start) {
            None => continue,
            Some(traversal) => return traversal,
        }
    }
    unreachable!("No valid path found")
}
fn part1(input: &str) -> usize {
    get_traversal(input).len() / 2
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    assert_eq!(part1(input), 8);
    let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    assert_eq!(part1(input), 8);
    let input = ".....
.S-7.
.|.|.
.L-J.
.....";
    assert_eq!(part1(input), 4);
}

fn part2(input: &str) -> usize {
    let traversal = get_traversal(input);
    let start = traversal[0];
    let include_s = start.1 != 0
        && (traversal[1] == (start.0, start.1 - 1)
            || traversal[traversal.len() - 2] == (start.0, start.1 - 1));
    let traversal: HashSet<(usize, usize)> = traversal.into_iter().collect();
    let mut inside_count: usize = 0;
    for (y, line) in input.lines().enumerate() {
        let mut inside = false;
        for (x, ch) in line.chars().enumerate() {
            if traversal.get(&(x, y)).is_some() {
                if ch != '-' && ch != '7' && ch != 'F' && (ch != 'S' || include_s) {
                    inside = !inside;
                }
            } else if inside {
                inside_count += 1;
            }
        }
    }
    inside_count
}

#[cfg(test)]
#[test]
fn test_part2() {
    let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    assert_eq!(part2(input), 4);
    let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    assert_eq!(part2(input), 10);
}
