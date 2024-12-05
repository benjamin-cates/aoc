use std::collections::HashMap;
fn main() {
    let input: &str = include_str!("../data/08.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let instruction: &str = input.lines().next().unwrap();
    let mut directions: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in input.lines().skip(2) {
        directions.insert(&line[0..3], (&line[7..10], &line[12..15]));
    }
    let mut cur = "AAA";
    let mut steps = 0;
    loop {
        for dir in instruction.chars() {
            if dir == 'R' {
                cur = directions.get(cur).unwrap().1
            } else {
                cur = directions.get(cur).unwrap().0
            }
            steps += 1;
            if cur == "ZZZ" {
                return steps;
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(part1(input), 2);
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first.max(second);
    let mut min = first.min(second);
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}

fn part2(input: &str) -> u64 {
    let instruction: &str = input.lines().next().unwrap();
    let mut directions: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in input.lines().skip(2) {
        directions.insert(&line[0..3], (&line[7..10], &line[12..15]));
    }
    let mut cur: Vec<&str> = vec![];
    for (loc, _) in directions.iter() {
        if loc.bytes().nth(2).unwrap() == b'A' {
            cur.push(loc);
        }
    }
    let mut loop_len: Vec<Option<u64>> = vec![None; cur.len()];
    let mut steps = 0;
    loop {
        for dir in instruction.chars() {
            steps += 1;
            for (i, loc) in cur.iter_mut().enumerate() {
                let options = directions.get(loc).unwrap();
                *loc = if dir == 'L' { options.0 } else { options.1 };
                // If ends in Z, write loop length
                if loc.bytes().nth(2).unwrap() == b'Z' {
                    if loop_len[i].is_none() {
                        loop_len[i] = Some(steps);
                    } else if steps % loop_len[i].unwrap() != 0 {
                        panic!("assumption broken {} {}", steps, loop_len[i].unwrap());
                    }
                }
            }
        }
        // If all loop lengths found, return least common multiple
        if loop_len.iter().all(|x| x.is_some()) {
            return loop_len.iter().fold(1, |a, b| lcm(a, b.unwrap()));
        }
    }
}

#[cfg(test)]
#[test]
fn test_part2() {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    assert_eq!(part2(input), 6);
}
