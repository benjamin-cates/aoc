use aoc24::CharGrid;

fn main() {
    let input: &str = include_str!("../data/04.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

// Finished in 11:41
fn part1(input: &str) -> usize {
    let width = input.lines().nth(0).unwrap().len();
    let height = input.lines().count();
    // The initial spaces prevent a false positive when wrapping around
    // Each 2d array is going to be flattened later
    let mut diagonals_slash = vec![vec![' ']; width + height];
    let mut diagonals_backslash = vec![vec![' ']; width + height];
    let mut rows = vec![vec![' ']; height];
    let mut cols = vec![vec![' ']; width];
    for (line_idx, line) in input.lines().enumerate() {
        for (ch_idx, ch) in line.chars().enumerate() {
            rows[line_idx].push(ch);
            cols[ch_idx].push(ch);
            diagonals_backslash[line_idx + ch_idx].push(ch);
            diagonals_slash[line_idx + width - ch_idx].push(ch);
        }
    }
    // Flatten the whole thing into a long series of slices
    let char_list: Vec<char> = rows.into_iter()
        .chain(cols.into_iter())
        .chain(diagonals_backslash.into_iter())
        .chain(diagonals_slash.into_iter())
        .flatten()
        .collect::<Vec<char>>();

    let xmas = &['X', 'M', 'A', 'S'];
    let samx = &['S', 'A', 'M', 'X'];
    char_list
        .windows(4)
        .filter(|v| v == xmas || v == samx)
        .count()
}

// Finished in 24:28
fn part2(input: &str) -> usize {
    let char_grid: CharGrid = CharGrid::new(input);
    let mut count = 0;
    // Iterate over possible centers of the X-MAS
    for i in 1..char_grid.width() - 1 {
        for j in 1..char_grid.height() - 1 {
            // All X-MAS crosses must have A at the center
            if char_grid.get((i, j)) != Some('A') {
                continue;
            }
            // Tuple with the pattern
            // 1 . 2
            // . . .
            // 3 . 4
            // Matches only if opposite diagonals are S and M
            count += match (
                char_grid.get((i - 1, j - 1)).unwrap(),
                char_grid.get((i - 1, j + 1)).unwrap(),
                char_grid.get((i + 1, j - 1)).unwrap(),
                char_grid.get((i + 1, j + 1)).unwrap(),
            ) {
                ('M', 'S', 'M', 'S') => 1,
                ('S', 'M', 'S', 'M') => 1,
                ('M', 'M', 'S', 'S') => 1,
                ('S', 'S', 'M', 'M') => 1,
                _ => 0,
            }
        }
    }
    count
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    assert_eq!(part1(input), 18);
    assert_eq!(part2(input), 9);
}
