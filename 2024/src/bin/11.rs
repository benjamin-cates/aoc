use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("../data/11.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} ({:?})", part1_new(input), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} ({:?})", part2(input), now.elapsed());
}
/// Takes map of (rock_num, count) and returns the map after blinking
fn blink_map(map: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut out = HashMap::new();
    for (val, count) in map {
        if *val == 0 {
            *out.entry(1).or_insert(0) += *count;
            continue;
        }

        let num_digits = val.ilog(10) + 1;
        if num_digits % 2 == 0 {
            let pow = (10usize).pow(num_digits / 2);
            *out.entry(val / pow).or_insert(0) += *count;
            *out.entry(val % pow).or_insert(0) += *count;
            continue;
        }

        *out.entry(2024 * val).or_insert(0) += *count;
    }
    out
}

fn blink(vec: &Vec<usize>) -> Vec<usize> {
    let mut out = vec![];
    for val in vec {
        if *val == 0 {
            out.push(1);
        } else if val.ilog(10) % 2 == 1 {
            let formatted = format!("{}", val);
            out.push(formatted[0..formatted.len() / 2].parse().unwrap());
            out.push(formatted[formatted.len() / 2..].parse().unwrap());
        } else {
            out.push(2024 * val);
        }
    }
    out
}

// 7579950253764 too high
// 6:44
fn part1(input: &str) -> usize {
    let mut vec: Vec<usize> = input
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    for _ in 0..25 {
        vec = blink(&vec);
    }
    vec.len()
}

fn part1_new(input: &str) -> usize {
    let mut map: HashMap<usize, usize> = input
        .split_whitespace()
        .map(|v| (v.parse().unwrap(), 1))
        .collect();
    for _ in 0..25 {
        map = blink_map(&map);
    }
    map.iter().map(|(_k, v)| v).sum()
}

// Finished in 15:55
fn part2(input: &str) -> usize {
    let mut map: HashMap<usize, usize> = input
        .split_whitespace()
        .map(|v| (v.parse().unwrap(), 1))
        .collect();
    for _ in 0..75 {
        map = blink_map(&map);
    }
    map.iter().map(|(_k, v)| v).sum()
}

#[cfg(test)]
#[test]
fn test_example() {
    let input = "";
    assert_eq!(part1(input), 0);
    assert_eq!(part2(input), 0);
}
