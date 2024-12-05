use std::collections::{HashMap, HashSet};

fn main() {
    let input: &str = include_str!("../data/05.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

struct Ordering(pub HashMap<usize, HashSet<usize>>);

impl Ordering {
    fn allowed_order(&self, a: usize, b: usize) -> bool {
        if let Some(required_before) = self.0.get(&a) {
            if required_before.contains(&b) {
                return false;
            }
        }
        return true;
    }
}

fn is_valid(vec: &Vec<usize>, ordering: &Ordering) -> bool {
    let mut after_list: HashSet<usize> = HashSet::new();
    for num in vec.iter().rev() {
        if let Some(required_before) = ordering.0.get(num) {
            if required_before.intersection(&after_list).count() != 0 {
                return false;
            }
        }
        after_list.insert(*num);
    }
    true
}

fn parse_rules(input: &str) -> Ordering {
    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    for line in input.lines() {
        let (first, second) = line.split_once("|").unwrap();
        let first = first.parse::<usize>().unwrap();
        let second = second.parse::<usize>().unwrap();
        match rules.get_mut(&second) {
            Some(set) => {
                set.insert(first);
            }
            None => {
                rules.insert(second, HashSet::from([first]));
            }
        }
    }
    Ordering(rules)
}

// Finished in 22:27
fn part1(input: &str) -> usize {
    let rules = parse_rules(input);
    let mut count = 0;
    for line in input.split("\n\n").nth(1).unwrap().lines() {
        let nums = line
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        if is_valid(&nums, &rules) {
            count += nums[nums.len() / 2];
        }
    }
    count
}

// Finished in 31:49
fn part2(input: &str) -> usize {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rules_str);
    let mut count = 0;
    for line in input.split("\n\n").nth(1).unwrap().lines() {
        let mut nums = line
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        if is_valid(&nums, &rules) {
            continue;
        }
        nums.sort_by(|a, b| {
            if !rules.allowed_order(*a, *b) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });
        count += nums[nums.len() / 2];
    }
    count
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    assert_eq!(part1(input), 143);
    assert_eq!(part2(input), 123);
}
