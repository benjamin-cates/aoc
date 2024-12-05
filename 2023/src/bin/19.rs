use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input: &str = include_str!("../data/19.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

/// A part is a list of numbers representing x, m, a, and s
type Part = HashMap<char, i32>;

/// An action is either accept, reject, or next workflow
enum Action<'a> {
    NextWorkflow(&'a str),
    Reject,
    Accept,
}

impl Action<'_> {
    /// "A" maps to accept, "R" maps to reject, all other are workflow names
    fn from_str<'a>(action: &'a str) -> Action<'a> {
        match action {
            "R" => Action::Reject,
            "A" => Action::Accept,
            workflow => Action::NextWorkflow(workflow),
        }
    }
}

/// A rule compares the number assigned to char ch with the number num.
/// If it is equivalent to the ordering Order, then apply the action
/// Exmaple rule:   x<1230:A    means Accept if x is less than 1230
/// And would be stored as
/// Rule {ch: 'x', ordering: Ordering::Less, num: 1230, action: Action::Accept}
struct Rule<'a> {
    ch: char,
    order: Ordering,
    num: i32,
    action: Action<'a>,
}

/// Parses a "workflow", this is a list of rules followed by a default action
fn parse_workflow<'a>(line: &'a str) -> (&'a str, (Vec<Rule<'a>>, Action<'a>)) {
    let name: &'a str = line.split("{").nth(0).unwrap();
    let mut parts = line.split("{").nth(1).unwrap();
    parts = &parts[0..(parts.len() - 1)];
    let default_action: Action<'a> = Action::from_str(parts.split(",").last().unwrap());
    let mut list = vec![];
    for command in parts.split(",").take(parts.split(",").count() - 1) {
        list.push(Rule {
            ch: command.chars().nth(0).unwrap(),
            order: match command.chars().nth(1).unwrap() {
                '<' => Ordering::Less,
                '>' => Ordering::Greater,
                ch => panic!("Invalid ordering {}", ch),
            },
            num: command.split(":").nth(0).unwrap()[2..].parse().unwrap(),
            action: Action::from_str(command.split(":").nth(1).unwrap()),
        });
    }
    (name, (list, default_action))
}

/// Parses a part that's stored as the assigmnet of the chars x, m, a, and s to their own number
/// Returns a hash map from character to its number
fn parse_part(line: &str) -> Part {
    line[1..(line.len() - 1)]
        .split(",")
        .map(|assign| {
            (
                assign.chars().nth(0).unwrap(),
                assign.split("=").nth(1).unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let new_line = input.lines().position(|x| x == "").unwrap();
    let workflows: HashMap<&str, (Vec<Rule>, Action)> =
        input.lines().take(new_line).map(parse_workflow).collect();
    let parts: Vec<Part> = input.lines().skip(new_line + 1).map(parse_part).collect();
    let mut passing_parts: i32 = 0;
    // Loop through all parts that are given in the problem
    'part_loop: for part in parts {
        let mut workflow = "in";
        // Continuously loop through workflows until accept or reject is reached
        'workflow_loop: loop {
            let (rules, default_action) = workflows.get(workflow).unwrap();
            // Loop through rules in a workflow
            for rule in rules {
                if part.get(&rule.ch).unwrap().cmp(&rule.num) == rule.order {
                    match rule.action {
                        Action::Reject => {
                            break 'workflow_loop;
                        }
                        Action::Accept => {
                            passing_parts += part.iter().map(|(_, val)| val).sum::<i32>();
                            continue 'part_loop;
                        }
                        Action::NextWorkflow(next) => {
                            workflow = next;
                            continue 'workflow_loop;
                        }
                    }
                }
            }
            // If non of the rules matched, do default action
            match default_action {
                Action::Reject => {
                    break 'workflow_loop;
                }
                Action::Accept => {
                    passing_parts += part.iter().map(|(_, val)| val).sum::<i32>();
                    break 'workflow_loop;
                }
                Action::NextWorkflow(next) => {
                    workflow = next;
                    continue 'workflow_loop;
                }
            }
        }
    }
    passing_parts as usize
}

/// PartSet is stored as mapping from chars to range [min,max)
type PartSet = HashMap<char, (i32, i32)>;
fn part_set_size(set: &PartSet) -> usize {
    set.iter()
        .map(|(_, (min, max))| (max - min) as usize)
        .product()
}
fn part2(input: &str) -> usize {
    let new_line = input.lines().position(|x| x == "").unwrap();
    let workflows: HashMap<&str, (Vec<Rule>, Action)> =
        input.lines().take(new_line).map(parse_workflow).collect();
    let mut passing_parts: usize = 0;
    // Part ranges to be processed, starts with all possible parts in the "in" workflow
    let mut parts_stack: Vec<(&str, PartSet)> = vec![(
        "in",
        HashMap::from([
            ('x', (1, 4001)),
            ('m', (1, 4001)),
            ('a', (1, 4001)),
            ('s', (1, 4001)),
        ]),
    )];
    while let Some((workflow, part)) = parts_stack.pop() {
        let (rules, default_action) = workflows.get(workflow).unwrap();
        let mut cur_part = part.clone();
        for rule in rules {
            if part_set_size(&cur_part) == 0 {
                break;
            }
            let (min, max) = cur_part.get(&rule.ch).unwrap().clone();
            // Create passing ranges and (non-passing) cur_part ranges
            let mut passing = cur_part.clone();
            let num = if rule.order == Ordering::Less {
                rule.num
            } else {
                rule.num + 1
            }
            .clamp(min, max);
            *passing.get_mut(&rule.ch).unwrap() = (min, num);
            *cur_part.get_mut(&rule.ch).unwrap() = (num, max);
            if rule.order == Ordering::Greater {
                std::mem::swap(&mut passing, &mut cur_part);
            }
            // What to do with passing set
            match rule.action {
                Action::Reject => {}
                Action::Accept => {
                    passing_parts += part_set_size(&passing);
                }
                Action::NextWorkflow(next) => {
                    if part_set_size(&passing) > 0 {
                        parts_stack.push((next, passing));
                    }
                }
            }
            // Non passing set is looped back to the next rule
        }
        // Parts that did not pass any rules are on the default action
        match default_action {
            Action::Reject => {}
            Action::Accept => {
                passing_parts += part_set_size(&cur_part);
            }
            Action::NextWorkflow(next) => {
                if part_set_size(&cur_part) > 0 {
                    parts_stack.push((next, cur_part));
                }
            }
        }
    }
    passing_parts
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    assert_eq!(part1(input), 19114);
    assert_eq!(part2(input), 167409079868000);
}
