use std::collections::{BTreeSet, HashMap};

fn main() {
    let input: &str = include_str!("../data/19.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} ({:?})", part1(input), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} ({:?})", part2(input), now.elapsed());
}
fn num_designs<'a>(
    design: &'a str,
    avail_patterns: &BTreeSet<&str>,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.len() == 0 {
        return 1;
    }
    if let Some(val) = memo.get(design) {
        return *val;
    }
    let mut sum = 0;
    for i in 1..=(8.min(design.len())) {
        if avail_patterns.get(&design[0..i]).is_some() {
            sum += num_designs(&design[i..], avail_patterns, memo);
        }
    }
    memo.insert(design, sum);
    return sum;
}

// Finished in 22:47
fn part1(input: &str) -> usize {
    let (avail_patterns, designs) = input.split_once("\n\n").unwrap();
    let avail_patterns = avail_patterns.split(", ").collect::<BTreeSet<&str>>();
    let mut memo: HashMap<&str, usize> = HashMap::new();
    designs
        .lines()
        .map(|design| num_designs(design, &avail_patterns, &mut memo) != 0)
        .count()
}

// Finished in 26:36
fn part2(input: &str) -> usize {
    let (avail_patterns, designs) = input.split_once("\n\n").unwrap();
    let avail_patterns = avail_patterns.split(", ").collect::<BTreeSet<&str>>();
    let mut memo: HashMap<&str, usize> = HashMap::new();
    designs
        .lines()
        .map(|design| num_designs(design, &avail_patterns, &mut memo))
        .sum::<usize>()
}

#[cfg(test)]
#[test]
fn test_example() {
    let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    assert_eq!(part1(input), 6);
    assert_eq!(part2(input), 0);
}
