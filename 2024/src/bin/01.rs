use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("../data/01.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut list_1: Vec<usize> = Vec::new();
    let mut list_2: Vec<usize> = Vec::new();
    for line in input.lines() {
        let vals = line
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        list_1.push(vals[0]);
        list_2.push(vals[1]);
    }
    (list_1, list_2)
}

fn part1(input: &str) -> usize {
    let (mut list_1, mut list_2) = parse_input(input);
    list_1.sort();
    list_2.sort();

    list_1
        .iter()
        .zip(list_2.iter())
        .fold(0, |a, b| a + b.0.abs_diff(*b.1))
}

fn part2(input: &str) -> usize {
    let (list_1, list_2) = parse_input(input);
    let mut counts_2 = HashMap::<usize, usize>::new();
    for val in list_2 {
        match counts_2.get_mut(&val) {
            Some(count) => *count += 1,
            None => {
                counts_2.insert(val, 1);
            }
        }
    }

    list_1
        .into_iter()
        .map(|v: usize| ((counts_2.get(&v).cloned().unwrap_or(0)) * v))
        .fold(0, |a, b| a + b)
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(part1(input), 11);
    assert_eq!(part2(input), 0);
}
