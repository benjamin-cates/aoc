use std::collections::HashSet;

use aoc24::{CharGrid, Direction, Point};

fn main() {
    let input: &str = include_str!("../data/15.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} ({:?})", part1(input), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} ({:?})", part2(input), now.elapsed());
}

// Finished in 10:31
fn part1(input: &str) -> usize {
    let (grid, directions) = input.split_once("\n\n").unwrap();
    let mut grid = CharGrid::new(grid);
    let directions = directions
        .chars()
        .filter_map(|dir| dir.try_into().ok())
        .collect::<Vec<Direction>>();

    let mut player = grid.find('@').unwrap();
    for dir in directions.iter() {
        let mut i = 1;
        let mut can_move = true;
        loop {
            match grid.get(player + dir.step() * i) {
                Some('O') => {
                    i += 1;
                }
                Some('#') => {
                    can_move = false;
                    break;
                }
                Some('.') => {
                    break;
                }
                _ => {
                    panic!("OOpsie");
                }
            };
        }
        if can_move {
            grid.set(player, '.');
            for i in 1.. {
                let cur = grid.get(player + dir.step() * i).unwrap();
                grid.set(player + dir.step() * i, if i == 1 { '@' } else { 'O' })
                    .unwrap();
                if cur != 'O' {
                    break;
                }
            }
            player = player + dir.step();
        }
    }
    let box_positions = grid.find_all('O');
    box_positions
        .map(|v| (v.x + 100 * v.y) as usize)
        .sum::<usize>()
}
fn move_set(
    grid: &CharGrid,
    moves: &mut HashSet<Point>,
    pos: Point,
    dir: Direction,
    split_box: bool,
) -> Option<()> {
    if dir == Direction::North || dir == Direction::South {
        if split_box && grid.get(pos) == Some('[') && moves.get(&(pos + (1, 0).into())).is_none() {
            move_set(grid, moves, pos, dir, false)?;
            return move_set(grid, moves, pos + (1, 0).into(), dir, false);
        }
        if split_box && grid.get(pos) == Some(']') && moves.get(&(pos + (-1, 0).into())).is_none() {
            move_set(grid, moves, pos, dir, false)?;
            return move_set(grid, moves, pos + (-1, 0).into(), dir, false);
        }
    }
    if grid.get(pos + dir.step()) == Some('#') {
        return None;
    }
    moves.insert(pos + dir.step());
    if grid.get(pos + dir.step()) == Some('.') {
        return Some(());
    }
    move_set(grid, moves, pos + dir.step(), dir, true)
}

// Finished in 1:20:47
fn part2(input: &str) -> usize {
    let (grid, directions) = input.split_once("\n\n").unwrap();
    let mut grid = CharGrid::new(grid);
    let directions = directions
        .chars()
        .filter_map(|dir| dir.try_into().ok())
        .collect::<Vec<Direction>>();
    grid.grid = grid
        .grid
        .into_iter()
        .map(|v| {
            v.iter()
                .map(|v| {
                    if *v == '.' {
                        ['.', '.'].into_iter()
                    } else if *v == '@' {
                        ['@', '.'].into_iter()
                    } else if *v == 'O' {
                        ['[', ']'].into_iter()
                    } else {
                        ['#', '#'].into_iter()
                    }
                })
                .flatten()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    let mut player = grid.find('@').unwrap();
    for dir in directions.iter() {
        let mut moves: HashSet<Point> = HashSet::new();
        if move_set(&grid, &mut moves, player, *dir, true).is_some() {
            let mut new_grid = grid.clone();
            for point in moves.iter() {
                let old_char = grid.get(*point + dir.opposite().step()).unwrap();
                if !moves.contains(&(*point + dir.opposite().step())) {
                    new_grid.set(*point + dir.opposite().step(), '.').unwrap();
                }
                new_grid.set(*point, old_char).unwrap();
            }
            //println!("{}", new_grid);
            new_grid.set(player, '.').unwrap();
            //std::thread::sleep(Duration::from_millis(200));
            player = player + dir.step();
            grid = new_grid;
        }
    }
    println!("{}", grid);
    let box_positions = grid.find_all('[');
    box_positions
        .map(|v| (v.x + 100 * v.y) as usize)
        .sum::<usize>()
}

#[cfg(test)]
#[test]
fn test_example() {
    let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    assert_eq!(part1(input), 10092);
    assert_eq!(part2(input), 9021);
}
