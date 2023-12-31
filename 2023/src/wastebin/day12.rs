fn main() {
    let input: &str = include_str!("../bin/12.txt");
    println!("Answer to part1: {}", part1(input));
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum State {
    Operational,
    Broken,
    Unknown,
}
use State::*;

impl State {
    fn matches(known: &Vec<State>, unknown: &Vec<State>) -> bool {
        known
            .iter()
            .zip(unknown.iter())
            .all(|(known, unknown)| match (known, unknown) {
                (Operational, Broken) => false,
                (Broken, Operational) => false,
                _ => true,
            })
    }
}

fn gen_space_combinations(spaces: usize, remaining_gaps: usize) -> Vec<Vec<usize>> {
    if remaining_gaps == 1 {
        return vec![vec![spaces]];
    }
    let mut combinations = vec![];
    if remaining_gaps - 1 > spaces {
        return vec![];
    }
    for i in 1..=(spaces + 2 - remaining_gaps) {
        let with_i = gen_space_combinations(spaces - i, remaining_gaps - 1);
        for mut comb in with_i.into_iter() {
            comb.push(i);
            combinations.push(comb);
        }
    }
    return combinations;
}

fn generate_combinations(broken_lens: Vec<usize>, line_len: usize) -> Vec<Vec<State>> {
    let extra_space: usize = line_len - broken_lens.iter().sum::<usize>();
    let mut combinations = gen_space_combinations(extra_space, broken_lens.len() + 1);
    combinations.append(
        &mut gen_space_combinations(extra_space, broken_lens.len())
            .into_iter()
            .map(|mut x| {
                x.push(0);
                x
            })
            .collect(),
    );
    combinations
        .into_iter()
        .map(|working_lens| {
            let mut out: Vec<State> = vec![];
            for i in 0..broken_lens.len() {
                out.append(&mut vec![Operational; working_lens[i]]);
                out.append(&mut vec![Broken; broken_lens[i]]);
            }
            out.append(&mut vec![Operational; *working_lens.last().unwrap()]);
            out
        })
        .collect()
}

fn get_combinations(line: &str, repetition: usize) -> usize {
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
    let combinations = generate_combinations(broken_lens, states.len());
    combinations
        .iter()
        .filter(|x| State::matches(x, &states))
        .count()
}

fn part1(input: &str) -> usize {
    input.lines().map(|line| get_combinations(line, 1)).sum()
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    println!("{:?}", generate_combinations(vec![1], 5));
    assert_eq!(part1(input), 21);
    assert_eq!(part1("????? 1"), 5);
    assert_eq!(part1("?#??? 1"), 1);
    assert_eq!(part1("?.??? 1"), 4);
    assert_eq!(part1("?.??? 1,1"), 4);
}

fn depth_first(known_states: &[State], broken_lens: &[usize], cur_broken_len: usize) -> usize {
    if cur_broken_len == broken_lens[0] {
        if known_states.len() >= 1 && known_states[0] == Broken {
            return 0;
        }
        if broken_lens.len() == 1 {
            return 1;
        }
        if known_states.len() >= 1 {
            return depth_first(&known_states[1..], &broken_lens[1..], 0);
        } else {
            return 0;
        }
    }
    if cur_broken_len != 0 {
        if known_states.len() == 0 || known_states[0] == Operational {
            return 0;
        }
        return depth_first(&known_states[1..], broken_lens, cur_broken_len + 1);
    }
    if known_states.len() == 0 {
        return 0;
    }
    if known_states[0] == Unknown {
        return depth_first(&known_states[1..], broken_lens, 0)
            + depth_first(&known_states[1..], broken_lens, 1);
    } else if known_states[0] == Broken {
        return depth_first(&known_states[1..], broken_lens, 1);
    } else if known_states[0] == Operational {
        return depth_first(&known_states[1..], broken_lens, 0);
    }
    0
}
