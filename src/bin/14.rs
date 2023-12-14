fn main() {
    let input: &str = include_str!("14.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RockState {
    Empty,
    SquareRock,
    SlidingRock,
}
use RockState::*;

fn slide_north(grid: &mut Vec<Vec<RockState>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != SlidingRock {
                continue;
            }
            grid[y][x] = Empty;
            for y_new in (0..=y).rev() {
                if y_new == 0 {
                    grid[0][x] = SlidingRock;
                    break;
                }
                if grid[y_new - 1][x] != Empty {
                    grid[y_new][x] = SlidingRock;
                    break;
                }
            }
        }
    }
}
fn slide_south(grid: &mut Vec<Vec<RockState>>) {
    for y in (0..grid.len()).rev() {
        for x in 0..grid[0].len() {
            if grid[y][x] != SlidingRock {
                continue;
            }
            grid[y][x] = Empty;
            for y_new in y..grid.len() {
                if y_new == grid.len() - 1 {
                    grid[y_new][x] = SlidingRock;
                    break;
                }
                if grid[y_new + 1][x] != Empty {
                    grid[y_new][x] = SlidingRock;
                    break;
                }
            }
        }
    }
}

fn slide_east(grid: &mut Vec<Vec<RockState>>) {
    for y in 0..grid.len() {
        for x in (0..grid[0].len()).rev() {
            if grid[y][x] != SlidingRock {
                continue;
            }
            grid[y][x] = Empty;
            for x_new in x..grid[0].len() {
                if x_new == grid[0].len() - 1 {
                    grid[y][x_new] = SlidingRock;
                    break;
                }
                if grid[y][x_new + 1] != Empty {
                    grid[y][x_new] = SlidingRock;
                    break;
                }
            }
        }
    }
}

fn slide_west(grid: &mut Vec<Vec<RockState>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != SlidingRock {
                continue;
            }
            grid[y][x] = Empty;
            for x_new in (0..=x).rev() {
                if x_new == 0 {
                    grid[y][0] = SlidingRock;
                    break;
                }
                if grid[y][x_new - 1] != Empty {
                    grid[y][x_new] = SlidingRock;
                    break;
                }
            }
        }
    }
}

fn get_rocks(input: &str) -> Vec<Vec<RockState>> {
    input
        .lines()
        .map(|x| {
            x.chars()
                .map(|ch| match ch {
                    '.' => Empty,
                    '#' => SquareRock,
                    'O' => SlidingRock,
                    _ => panic!("Invalid char {}", ch),
                })
                .collect()
        })
        .collect()
}

fn count_weight_north(rocks: &Vec<Vec<RockState>>) -> usize {
    rocks
        .iter()
        .enumerate()
        .map(|(idx, line)| (rocks.len() - idx) * line.iter().filter(|x| x == &&SlidingRock).count())
        .sum()
}

fn part1(input: &str) -> usize {
    let mut rocks = get_rocks(input);
    slide_north(&mut rocks);
    count_weight_north(&rocks)
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    assert_eq!(part1(input), 136);
}

fn slide_cycle(rocks: &mut Vec<Vec<RockState>>) {
    slide_north(rocks);
    slide_west(rocks);
    slide_south(rocks);
    slide_east(rocks);
}

fn part2(input: &str) -> usize {
    let mut rocks = get_rocks(input);
    let mut rocks_checkpoint = vec![];
    let mut i = 0;
    loop {
        slide_cycle(&mut rocks);
        i += 1;
        if rocks_checkpoint == rocks {
            break;
        }
        if i == 200 {
            rocks_checkpoint = rocks.clone();
        }
    }
    let remaining = (1000000000 - 200) % (i - 200);
    for _ in 0..remaining {
        slide_cycle(&mut rocks);
    }
    count_weight_north(&rocks)
}

#[cfg(test)]
#[test]
fn test_part2() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    assert_eq!(part2(input), 64);
}
