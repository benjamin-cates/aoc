use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input: &str = include_str!("../data/22.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} ({:?})", part1(input), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} ({:?})", part2(input), now.elapsed());
}

#[inline]
fn next_sec_num(mut sec: usize) -> usize {
    sec = (sec ^ (sec * 64)) % 16777216;
    sec = (sec ^ (sec / 32)) % 16777216;
    sec = (sec ^ (sec * 2048)) % 16777216;
    sec
}

// Finished in 5:50
fn part1(input: &str) -> usize {
    let mut sum = 0;
    for mut num in input.lines().map(|line| line.parse().unwrap()) {
        for _ in 0..2000 {
            num = next_sec_num(num);
        }
        sum += num;
    }
    sum
}

/// Returns an iterator over the past four changes and the current price
fn windows_of_changes(init: usize) -> impl Iterator<Item=([i8;4],u8)> {
    let first = init;
    let second = next_sec_num(init);
    let third = next_sec_num(second);
    let fourth = next_sec_num(third);
    let mut prices: VecDeque<i8> = VecDeque::from([(first % 10) as i8, (second % 10) as i8, (third % 10) as i8, (fourth % 10) as i8]);
    let mut secret_num = fourth;
    let mut i = 4;
    std::iter::from_fn(move || {
        i += 1;
        if i == 2001 {
            return None;
        }
        let next_num = next_sec_num(secret_num);
        secret_num = next_num;
        prices.push_back((next_num % 10) as i8);
        let out = ([prices[1] - prices[0], prices[2]-prices[1], prices[3]-prices[2], prices[4]-prices[3]],(next_num % 10) as u8);
        prices.pop_front();
        Some(out)
    })
}

// Finished in 42:42
fn part2(input: &str) -> usize {
    let mut bananas: HashMap<[i8; 4], usize> = HashMap::new();
    let mut accessed: HashSet<[i8;4]> = HashSet::new();
    for num in input.lines().map(|line| line.parse::<usize>().unwrap()) {
        windows_of_changes(num).for_each(|(window, price)| {
            if accessed.insert(window) {
                *bananas.entry(window).or_default() += price as usize;
            }
        });
        accessed.clear();
    }
    *bananas.iter().map(|(_,b)| b).max().unwrap()
}

#[cfg(test)]
#[test]
fn test_example() {
    let input = "1
10
100
2024";
    assert_eq!(part1(input), 37327623);
    let input = "1
2
3
2024";
    assert_eq!(part2(input), 23);
}