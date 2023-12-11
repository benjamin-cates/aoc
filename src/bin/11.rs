fn main() {
    let input: &str = include_str!("11.txt");
    println!("Answer to part1: {}", dists(input, 1));
    println!("Answer to part2: {}", dists(input, 1_000_000));
}

/// Get list of galaxy positions
fn get_galaxies(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_col, ch)| *ch == '#')
                .map(move |(col, _)| (col, row))
        })
        .collect()
}

/// Returns a mapping from each row to it's new position after expansion of age years
fn row_mapping(lines: &Vec<&str>, age: usize) -> Vec<usize> {
    let empty_rows: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_row, line)| line.chars().all(|ch| ch == '.'))
        .map(|(row, _x)| row)
        .collect();
    create_mapping_from_empty(lines.len(), empty_rows, age)
}
/// Returns a mapping from each column to it's new position after expansion of age years
fn column_mapping(lines: &Vec<&str>, age: usize) -> Vec<usize> {
    let empty_cols: Vec<usize> = (0..lines[0].len())
        .filter(|col| (0..lines.len()).all(|y| lines[y].chars().nth(*col).unwrap() == '.'))
        .collect();
    create_mapping_from_empty(lines[0].len(), empty_cols, age)
}

/// Create a map from current index to future index after each item in empty_places is expanded to
/// be size age
fn create_mapping_from_empty(map_size: usize, empty_places: Vec<usize>, age: usize) -> Vec<usize> {
    let mut num_empty: usize = 0;
    (0..map_size)
        .map(|x| {
            if empty_places.get(num_empty) == Some(&x) {
                num_empty += 1;
            }
            x + num_empty * (age - 1)
        })
        .collect()
}

/// Get the sum distances between galaxies after age years
fn dists(input: &str, age: usize) -> usize {
    let galaxy_locations = get_galaxies(input);
    let lines: Vec<&str> = input.lines().collect();
    let row_map = row_mapping(&lines, age);
    let col_map = column_mapping(&lines, age);
    let mut cum_dist = 0;
    for i in 0..galaxy_locations.len() {
        for j in (i + 1)..galaxy_locations.len() {
            cum_dist += row_map[galaxy_locations[i].1].abs_diff(row_map[galaxy_locations[j].1])
                + col_map[galaxy_locations[i].0].abs_diff(col_map[galaxy_locations[j].0]);
        }
    }
    cum_dist
}

#[cfg(test)]
#[test]
fn test_dists() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    assert_eq!(dists(input, 2), 374);
    assert_eq!(dists(input, 10), 1030);
    assert_eq!(dists(input, 100), 8410);
}
