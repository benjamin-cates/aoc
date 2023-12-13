fn main() {
    let input: &str = include_str!("13.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Thing {
    Rock,
    Ash,
}

fn get_smudged_mirrors(input: Vec<Vec<Thing>>, smudge_count: usize) -> usize {
    let row_mir: usize = (1..input.len())
        .filter(|y| {
            let y = *y;
            let mut smudges = 0;
            for row in 0..y {
                if 2 * y - row - 1 >= input.len() {
                    continue;
                }
                for x in 0..input[0].len() {
                    if input[row][x] != input[2 * y - row - 1][x] {
                        smudges += 1;
                    }
                }
            }
            smudges == smudge_count
        })
        .sum();
    let col_mir: usize = (1..input[0].len())
        .filter(|x| {
            let x = *x;
            let mut smudges = 0;
            for col in 0..x {
                if 2 * x - col - 1 >= input[0].len() {
                    continue;
                }
                for y in 0..input.len() {
                    if input[y][col] != input[y][2 * x - col - 1] {
                        smudges += 1;
                    }
                }
            }
            smudges == smudge_count
        })
        .sum();
    100 * row_mir + col_mir
}

fn mirror_sum_with_smudges(input: &str, smudges: usize) -> usize {
    input
        .split("\n\n")
        .map(|block| {
            let grid: Vec<Vec<Thing>> = block
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|ch| match ch {
                            '#' => Thing::Rock,
                            '.' => Thing::Ash,
                            _ => panic!("Unexpected char '{}'", ch),
                        })
                        .collect()
                })
                .collect();
            get_smudged_mirrors(grid, smudges)
        })
        .sum()
}

fn part1(input: &str) -> usize {
    mirror_sum_with_smudges(input, 0)
}

fn part2(input: &str) -> usize {
    mirror_sum_with_smudges(input, 1)
}

#[cfg(test)]
#[test]
fn test_mirrors() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    assert_eq!(part1(input), 405);
    assert_eq!(part2(input), 400)
}
