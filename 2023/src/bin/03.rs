fn main() {
    let input: &str = include_str!("../data/03.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

struct PartNumber {
    row: usize,
    start: usize,
    end: usize,
    number: usize,
    is_included: bool,
}

fn collect_part_numbers(input: &str) -> Vec<PartNumber> {
    let lines: Vec<&str> = input.lines().collect();
    let mut part_list: Vec<PartNumber> = vec![];
    // Find all part numbers
    for (row, line) in input.lines().enumerate() {
        let mut start_pos: Option<usize> = None;
        let mut number: usize = 0;
        for (i, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                if start_pos.is_none() {
                    start_pos = Some(i);
                }
                number *= 10;
                number += char.to_digit(10).unwrap() as usize;
            } else if start_pos.is_some() {
                part_list.push(PartNumber {
                    row,
                    start: start_pos.unwrap(),
                    end: i - 1,
                    number,
                    is_included: false,
                });
                start_pos = None;
                number = 0;
            }
        }
        if start_pos.is_some() {
            part_list.push(PartNumber {
                row,
                start: start_pos.unwrap(),
                end: line.len() - 1,
                number,
                is_included: false,
            });
        }
    }
    part_list
}

impl PartNumber {
    fn is_next_to_symbol(&self, lines: &Vec<&str>) -> bool {
        let upper_bound: usize = self.row.max(1) - 1;
        let lower_bound: usize = (self.row + 1).min(lines.len() - 1);
        let left_bound: usize = self.start.max(1) - 1;
        let right_bound: usize = (self.end + 1).min(lines[0].len() - 1);
        for y in upper_bound..=lower_bound {
            for x in left_bound..=right_bound {
                let char = lines[y].bytes().skip(x).next().unwrap();
                if !char.is_ascii_digit() && char != b'.' {
                    return true;
                }
            }
        }
        return false;
    }
}

fn part1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut part_list: Vec<PartNumber> = collect_part_numbers(input);

    //Iterate over a bounding box around each part number to search for symbols
    for part in part_list.iter_mut() {
        part.is_included = part.is_next_to_symbol(&lines);
    }

    // Sum part numbers that are included
    part_list
        .into_iter()
        .filter(|x| x.is_included)
        .map(|x| x.number)
        .sum()
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(part1(input), 4361);
}

struct Gear {
    row: usize,
    column: usize,
    ratio: Option<usize>,
}

fn part2(input: &str) -> usize {
    let mut gear_list: Vec<Gear> = vec![];
    // Create list of gears
    for (row, line) in input.lines().enumerate() {
        for (column, char) in line.chars().enumerate() {
            if char == '*' {
                gear_list.push(Gear {
                    row,
                    column,
                    ratio: None,
                });
            }
        }
    }
    // Get part number list
    let part_list = collect_part_numbers(input);
    // For each gear, check if it is adjacent to exactly two part numbers
    for gear in gear_list.iter_mut() {
        let mut adjacent_numbers: Vec<usize> = vec![];
        for part in part_list.iter() {
            if gear.column <= part.end + 1
                && gear.column as i32 >= part.start as i32 - 1
                && gear.row <= part.row + 1
                && gear.row as i32 >= part.row as i32 - 1
            {
                adjacent_numbers.push(part.number);
            }
        }
        gear.ratio = Some(if adjacent_numbers.len() == 2 {
            adjacent_numbers.iter().product()
        } else {
            0
        })
    }
    gear_list.iter().map(|x| x.ratio.unwrap()).sum()
}

#[cfg(test)]
#[test]
fn test_part2() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(part2(input), 467835);
}
