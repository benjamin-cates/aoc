fn main() {
    let input: &str = include_str!("12.txt");
    println!("Answer to part1: {}", solve(input, 1));
    println!("Answer to part2: {}", solve(input, 5));
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum State {
    Operational,
    Broken,
    Unknown,
}
use State::*;

/// Parses line into map vector and vector of broken stretches
fn parse(line: &str, repetition: usize) -> (Vec<State>, Vec<usize>) {
    let states: Vec<State> = line
        .split(" ")
        .next()
        .unwrap()
        .chars()
        .map(|x| match x {
            '.' => Operational,
            '#' => Broken,
            '?' => Unknown,
            _ => unimplemented!(),
        })
        .collect();
    let states = vec![states; repetition].join(&Unknown);
    let broken_lens: Vec<usize> = line
        .split(" ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let broken_lens = vec![broken_lens; repetition]
        .into_iter()
        .flatten()
        .collect();
    (states, broken_lens)
}

/// Solves the puzzle using a number of repetitions
fn solve(input: &str, repetition: usize) -> usize {
    input
        .lines()
        .map(|line| {
            let (states, broken_lens) = parse(line, repetition);
            valid_combinations(states, broken_lens)
        })
        .sum()
}

/// Stores the current stretch of broken springs that it has passed
/// and the length of the broken stretch it is currently in
/// Each of these describes a branch state that will have the same number of
/// combinations further down the line
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Branch {
    broken_idx: usize,
    cur_broken_len: usize,
}
use std::collections::HashMap;

/// Add count to the key-value pair indexed by branch, or if it does not exist
/// add a new key-value pair
fn add(map: &mut HashMap<Branch, usize>, branch: Branch, count: usize) {
    match map.get_mut(&branch) {
        None => {
            map.insert(branch, count);
        }
        Some(reference) => *reference += count,
    };
}

/// Get number of valid combinations using a known state vector (map of springs)
/// And the list of each broken length
fn valid_combinations(state: Vec<State>, broken_lens: Vec<usize>) -> usize {
    let mut cur_iter = HashMap::new();
    cur_iter.insert(
        Branch {
            broken_idx: 0,
            cur_broken_len: 0,
        },
        1,
    );
    let mut next_iter: HashMap<Branch, usize> = HashMap::new();
    for i in 0..=state.len() {
        for (mut branch, count) in cur_iter.into_iter() {
            if branch.broken_idx == broken_lens.len() {
                if i == state.len() || state[i] != Broken {
                    add(&mut next_iter, branch, count);
                }
                continue;
            }
            if branch.cur_broken_len == broken_lens[branch.broken_idx] {
                if i != state.len() && state[i] == Broken {
                    continue;
                }
                branch.broken_idx += 1;
                branch.cur_broken_len = 0;
                add(&mut next_iter, branch, count);
                continue;
            }
            if branch.cur_broken_len != 0 {
                if i == state.len() || state[i] == Operational {
                    continue;
                }
                branch.cur_broken_len += 1;
                add(&mut next_iter, branch, count);
                continue;
            }
            if i == state.len() {
                add(&mut next_iter, branch, count);
                continue;
            }
            if state[i] == Unknown {
                branch.cur_broken_len = 0;
                add(&mut next_iter, branch, count);
                branch.cur_broken_len = 1;
                add(&mut next_iter, branch, count);
            } else if state[i] == Broken {
                branch.cur_broken_len = 1;
                add(&mut next_iter, branch, count);
            } else if state[i] == Operational {
                branch.cur_broken_len = 0;
                add(&mut next_iter, branch, count);
            }
        }
        cur_iter = next_iter;
        next_iter = HashMap::new();
    }
    cur_iter
        .into_iter()
        .filter(|(branch, _count_)| branch.broken_idx == broken_lens.len())
        .map(|(_, count)| count)
        .sum()
}

#[cfg(test)]
#[test]
fn test_solve() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    assert_eq!(solve(input, 1), 21);
    assert_eq!(solve(input, 5), 525152);
    assert_eq!(solve("????? 1", 1), 5);
    assert_eq!(solve("?#??? 1", 1), 1);
    assert_eq!(solve("?.??? 1", 1), 4);
    assert_eq!(solve("?.??? 1,1", 1), 4);
    assert_eq!(solve("#### 4", 1), 1);
    assert_eq!(solve("? 1", 1), 1);
    assert_eq!(solve("?. 1", 1), 1);
    assert_eq!(solve("?.# 1,1", 1), 1);
}
