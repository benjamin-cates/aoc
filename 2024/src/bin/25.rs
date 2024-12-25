use aoc24::CharGrid;

fn main() {
    let input: &str = include_str!("../data/25.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} ({:?})", part1(input), now.elapsed());
}

fn part1(input: &str) -> usize {
    let elements = input.split("\n\n").map(|block| {
        let grid = CharGrid::new(block.trim().trim_ascii());
        if grid.grid[0][0] == '#' {
            let heights = [0,1,2,3,4].map(|col|{
                for i in 0.. {
                    if grid.grid[i][col] == '.' {
                        return i - 1;
                    }
                }
                0
            });
            (false, heights)
        }
        else {
            let heights = [0,1,2,3,4].map(|col|{
                for i in 0.. {
                    if grid.grid[6-i][col] == '.' {
                        return i - 1;
                    }
                }
                0
            });
            (true, heights)
        }
    }).collect::<Vec<(bool, [usize; 5])>>();
    let keys = elements.iter().filter(|(v,_)| *v).map(|(_, v)| *v).collect::<Vec<[usize;5]>>();
    let locks = elements.iter().filter(|(v,_)| !v).map(|(_, v)| *v).collect::<Vec<[usize;5]>>();
    let mut sum = 0;
    for key in &keys {
        for lock in &locks {
            if key.iter().enumerate().all(|(i, key_val)| key_val + lock[i] <= 5) {
                sum += 1;
            }
        }
    }
    sum
}

#[cfg(test)]
#[test]
fn test_example() {
    let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
    assert_eq!(part1(input), 3);
}
