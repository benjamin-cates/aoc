use std::collections::{HashMap, VecDeque};

fn main() {
    let input: &str = include_str!("20.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}
use Pulse::*;

#[derive(Clone, Debug)]
struct Module<'a> {
    connects_to: Vec<&'a str>,
    state: ModuleState<'a>,
}

#[derive(Clone, Debug)]
enum ModuleState<'a> {
    FlipFlop(Pulse),
    Conjunction(HashMap<&'a str, Pulse>),
    Input,
}

fn parse_connections<'a>(input: &'a str) -> HashMap<&'a str, Module<'a>> {
    let mut out = HashMap::new();
    // Get each module name, its type, and where it connects to
    for line in input.lines() {
        let to_list: Vec<&str> = line.split(" -> ").nth(1).unwrap().split(", ").collect();
        let name = match line.chars().nth(0).unwrap() {
            '&' => &line[1..],
            '%' => &line[1..],
            _ => line,
        }
        .split(" -> ")
        .nth(0)
        .unwrap();
        out.insert(
            name,
            Module {
                connects_to: to_list,
                state: match line.chars().nth(0).unwrap() {
                    '&' => ModuleState::Conjunction(HashMap::new()),
                    '%' => ModuleState::FlipFlop(Low),
                    _ => ModuleState::Input,
                },
            },
        );
    }
    // Link up all connections to conjunctions
    let out_copy = out.clone();
    for (name, module) in out_copy.iter() {
        for connection in module.connects_to.iter() {
            if let Some(Module {
                connects_to: _,
                state: ModuleState::Conjunction(map),
            }) = out.get_mut(connection)
            {
                map.insert(name, Low);
            }
        }
    }
    out
}

fn part1(input: &str) -> usize {
    let mut mapping = parse_connections(input);
    let mut pulse_queue: VecDeque<(&str, &str, Pulse)> = VecDeque::new();
    let mut high_count = 0;
    let mut low_count = 0;
    for _ in 0..1000 {
        pulse_queue.push_back(("broadcaster", "button", Low));
        while let Some((name, from, pulse)) = pulse_queue.pop_front() {
            if pulse == High {
                high_count += 1;
            } else {
                low_count += 1;
            }
            if let Some(Module {
                connects_to: connections,
                ref mut state,
            }) = mapping.get_mut(name)
            {
                let send: Option<Pulse> = match state {
                    ModuleState::Input => Some(pulse),
                    ModuleState::Conjunction(map) => {
                        *map.get_mut(from).unwrap() = pulse;
                        if map.iter().all(|(_k, val)| *val == High) {
                            Some(Low)
                        } else {
                            Some(High)
                        }
                    }
                    ModuleState::FlipFlop(state) => {
                        if pulse == Low {
                            *state = match *state {
                                Low => High,
                                High => Low,
                            };
                            Some(*state)
                        } else {
                            None
                        }
                    }
                };
                if let Some(pulse) = send {
                    for con in connections.iter() {
                        pulse_queue.push_back((con, name, pulse));
                    }
                }
            }
        }
    }
    high_count * low_count
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

fn part2(input: &str) -> usize {
    let mut mapping = parse_connections(input);
    let mut pulse_queue: VecDeque<(&str, &str, Pulse)> = VecDeque::new();
    let mut i = 0;
    let cycle_head = if let Some(head) = mapping
        .iter()
        .find(|(_name, module)| module.connects_to == vec!["rx"])
    {
        *head.0
    } else {
        panic!("Nothing feeds into rx")
    };
    let mut children =
        if let ModuleState::Conjunction(from) = &mapping.get(cycle_head).unwrap().state {
            from.iter()
                .map(|(name, _)| (*name, 0))
                .collect::<HashMap<&str, usize>>()
        } else {
            panic!("Nothing feeds into {}", cycle_head);
        };
    loop {
        i += 1;
        pulse_queue.push_back(("broadcaster", "button", Low));
        while let Some((name, from, pulse)) = pulse_queue.pop_front() {
            if name == cycle_head && pulse == High {
                *children.get_mut(&from).unwrap() = i;
                if children.iter().all(|(_name, x)| x != &0) {
                    return children
                        .iter()
                        .fold(1, |a, (_n, val)| lcm(a as u64, *val as u64))
                        as usize;
                }
            }
            if let Some(Module {
                connects_to: connections,
                ref mut state,
            }) = mapping.get_mut(name)
            {
                let send: Option<Pulse> = match state {
                    ModuleState::Input => Some(pulse),
                    ModuleState::Conjunction(map) => {
                        *map.get_mut(from).unwrap() = pulse;
                        if map.iter().all(|(_k, val)| *val == High) {
                            Some(Low)
                        } else {
                            Some(High)
                        }
                    }
                    ModuleState::FlipFlop(state) => {
                        if pulse == Low {
                            *state = match *state {
                                Low => High,
                                High => Low,
                            };
                            Some(*state)
                        } else {
                            None
                        }
                    }
                };
                if let Some(pulse) = send {
                    for con in connections.iter() {
                        pulse_queue.push_back((con, name, pulse));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    assert_eq!(part1(input), 32000000);
}
