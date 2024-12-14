use std::collections::HashSet;

use aoc24::{CharGrid, Direction, Point};

fn main() {
    let input: &str = include_str!("../data/06.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

// Finished in 9:30
fn part1(input: &str) -> usize {
    let grid = CharGrid::new(input);
    let soldier_pos = grid.find('^').unwrap();
    traverse(&grid, soldier_pos).unwrap().iter().count()
}

fn traverse(grid: &CharGrid, mut soldier_pos: Point) -> Option<HashSet<Point>> {
    let mut soldier_dir = Direction::North;
    let mut positions_set: HashSet<Point> = HashSet::from([soldier_pos]);
    let mut history_set: HashSet<(Point,Direction)> = HashSet::from([(soldier_pos,soldier_dir)]);
    loop {
        let next_pos = soldier_pos + soldier_dir.step();
        match grid.get(next_pos) {
            Some('#') => {
                soldier_dir = match soldier_dir {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                };
            }
            Some(_) => {
                if history_set.contains(&(next_pos,soldier_dir)) {
                    return None;
                }
                history_set.insert((next_pos,soldier_dir));
                positions_set.insert(next_pos);
                soldier_pos = next_pos;
            }
            None => {
                return Some(positions_set);
            }
        }
    }
}

// Finished in 20:08
fn part2(input: &str) -> usize {
    let mut grid = CharGrid::new(input);
    let soldier_pos = grid.find('^').unwrap();
    let normal_path = traverse(&grid, soldier_pos).unwrap();
    let mut count = 0;
    for x in 0..(grid.width() as usize) {
        for y in 0..(grid.height() as usize) {
            match grid.grid[y][x] {
                '.' => {
                    if !normal_path.contains(&Point::from((x,y))) {
                        continue;
                    }
                    grid.grid[y][x] = '#';
                    if traverse(&grid, soldier_pos).is_none() {
                        count += 1;
                    }
                    grid.grid[y][x] = '.';
                }
                _ => {

                }
            }

        }
    }
    count
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    assert_eq!(part1(input), 41);
    assert_eq!(part2(input), 6);
}