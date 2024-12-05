use std::collections::HashSet;
use std::collections::VecDeque;
fn main() {
    let input: &str = include_str!("../data/16.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Dir {
    North,
    South,
    East,
    West,
}
use Dir::*;

impl Dir {
    fn dx(&self) -> i32 {
        match self {
            East => 1,
            West => -1,
            _ => 0,
        }
    }
    fn dy(&self) -> i32 {
        match self {
            North => -1,
            South => 1,
            _ => 0,
        }
    }
}

fn part1(input: &str) -> usize {
    cast_laser(input, (0, 0), East)
}

fn cast_laser(input: &str, start: (i32, i32), start_dir: Dir) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut energized: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    let mut lasers: VecDeque<((i32, i32), Dir)> = vec![(
        (start.0 - start_dir.dx(), start.1 - start_dir.dy()),
        start_dir,
    )]
    .into();
    let mut prev_beams = HashSet::<((i32, i32), Dir)>::new();
    while !lasers.is_empty() {
        let laser = lasers.pop_back().unwrap();
        if prev_beams.get(&laser).is_some() {
            continue;
        }
        prev_beams.insert(laser);
        let pos = laser.0;
        let dir = laser.1;
        let new_pos = (pos.0 + dir.dx(), pos.1 + dir.dy());
        if new_pos.0 < 0
            || new_pos.0 as usize >= grid[0].len()
            || new_pos.1 < 0
            || new_pos.1 as usize >= grid.len()
        {
            continue;
        }
        energized[new_pos.1 as usize][new_pos.0 as usize] = true;
        match grid[new_pos.1 as usize][new_pos.0 as usize] {
            '/' => lasers.push_back((
                new_pos,
                match dir {
                    North => East,
                    South => West,
                    West => South,
                    East => North,
                },
            )),
            '\\' => lasers.push_back((
                new_pos,
                match dir {
                    North => West,
                    South => East,
                    East => South,
                    West => North,
                },
            )),
            '|' => match dir {
                South | North => lasers.push_back((new_pos, dir)),
                East | West => {
                    lasers.push_back((new_pos, North));
                    lasers.push_back((new_pos, South));
                }
            },
            '-' => match dir {
                East | West => lasers.push_back((new_pos, dir)),
                North | South => {
                    lasers.push_back((new_pos, East));
                    lasers.push_back((new_pos, West));
                }
            },
            _ => lasers.push_back((new_pos, dir)),
        }
    }

    energized.iter().flatten().filter(|x| **x).count()
}

fn part2(input: &str) -> usize {
    let mut energies = vec![];
    let width = input.lines().nth(0).unwrap().chars().count() as i32;
    let height = input.lines().count() as i32;
    for i in 0..width {
        energies.push(cast_laser(input, (i, 0), South));
        energies.push(cast_laser(input, (i, height - 1), North));
    }
    for i in 0..width {
        energies.push(cast_laser(input, (0, i), East));
        energies.push(cast_laser(input, (width - 1, i), West));
    }
    *energies.iter().max().unwrap()
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = ">|<<<\\....
|v-.\\^....
.v...|->>>
.v...v^.|.
.v...v^...
.v...v^..\\
.v../2\\\\..
<->-/vv|..
.|<<<2-|.\\
.v//.|.v..";
    assert_eq!(part1(input), 46);
    assert_eq!(part2(input), 0);
}
