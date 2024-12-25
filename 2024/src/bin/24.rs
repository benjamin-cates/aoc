use std::collections::{BTreeMap, HashMap};

fn main() {
    let input: &str = include_str!("../bin/message.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} ({:?})", part1(input), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} ({:?})", part2(input), now.elapsed());
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Gate {
    Xor,
    And,
    Or,
}

fn calculate<'a>(
    gate: &'a str,
    states: &mut HashMap<&'a str, bool>,
    rules: &HashMap<&'a str, (&'a str, Gate, &'a str)>,
) -> bool {
    let (left, rule, right) = rules.get(gate).unwrap();
    if !states.contains_key(left) {
        calculate(left, states, rules);
    }
    let left = *states.get(left).unwrap();
    if !states.contains_key(right) {
        calculate(right, states, rules);
    }
    let right = *states.get(right).unwrap();
    let out = match rule {
        Gate::Xor => right ^ left,
        Gate::And => right && left,
        Gate::Or => right || left,
    };
    states.insert(gate, out);
    out
}

// Finished in 21:01
fn part1(input: &str) -> usize {
    let (states, rules) = input.split_once("\n\n").unwrap();
    let mut states = states
        .lines()
        .map(|v| {
            (
                v.split_once(":").unwrap().0,
                if v.split_once(": ").unwrap().1 == "1" {
                    true
                } else {
                    false
                },
            )
        })
        .collect::<HashMap<&str, bool>>();
    let rules = rules
        .lines()
        .map(|line| {
            let terms = line.split(" ").collect::<Vec<&str>>();
            (
                terms[4],
                (
                    terms[0],
                    match terms[1] {
                        "XOR" => Gate::Xor,
                        "OR" => Gate::Or,
                        "AND" => Gate::And,
                        _ => panic!(""),
                    },
                    terms[2],
                ),
            )
        })
        .collect::<HashMap<&str, (&str, Gate, &str)>>();
    let mut out: String = String::new();
    for i in 0.. {
        let name = format!("z{i:0>2}");
        if !rules.contains_key(&name.as_str()) {
            break;
        }
        out.push_str(
            if calculate(
                rules.get_key_value(name.as_str()).unwrap().0,
                &mut states,
                &rules,
            ) {
                "1"
            } else {
                "0"
            },
        );
    }
    usize::from_str_radix(out.chars().rev().collect::<String>().as_str(), 2).unwrap()
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Role {
    X(usize),
    Y(usize),
    Intermediate(usize),
    CarryPart1(usize),
    CarryPart2(usize),
    Carry(usize),
    Z(usize),
}


fn rename_engine<'a>(
    gate: &'a str,
    role: &mut BTreeMap<&'a str, Role>,
    rules: &HashMap<&'a str, (&'a str, Gate, &'a str)>,
) -> Result<(), (&'a str, Role, Role)>{
    if role.contains_key(gate) {
        return Ok(());
    }
    if gate.starts_with("x") {
        role.insert(gate, Role::X(gate[1..].parse().unwrap()));
        return Ok(());
    }
    if gate.starts_with("y") {
        role.insert(gate, Role::Y(gate[1..].parse().unwrap()));
        return Ok(());
    }
    let (left, rule, right) = rules.get(&gate).unwrap();
    rename_engine(left, role, rules)?;
    rename_engine(right, role, rules)?;
    let left_role = *role.get(left).unwrap();
    let right_role = *role.get(right).unwrap();
    match (left_role, right_role) {
        (Role::X(0), Role::Y(0)) | (Role::Y(0), Role::X(0)) => {
            if *rule == Gate::Xor {
                if !gate.starts_with("z") {
                    return Err((gate, left_role, right_role));
                }
                role.insert(gate, Role::Z(0));
            }
            else if *rule == Gate::And {
                role.insert(gate, Role::Carry(0));
            }
            else {
                panic!("Oring two inputs is not expected");
            }
        }
        (Role::X(x), Role::Y(y)) | (Role::Y(y), Role::X(x)) if x == y => {
            if *rule == Gate::Xor {
                role.insert(gate, Role::Intermediate(x));
            }
            else if *rule == Gate::And {
                role.insert(gate, Role::CarryPart1(x));
            }
            else {
                panic!("Oring two inputs is not expected");
            }
        }

        (Role::CarryPart1(x), Role::CarryPart2(y)) | (Role::CarryPart2(y), Role::CarryPart1(x)) if x == y => {
            if *rule == Gate::Or {
                role.insert(gate, Role::Carry(x));
            }
            else {
                return Err((gate, left_role, right_role));
            }
        }
        (Role::Intermediate(i), Role::Carry(c)) | (Role::Carry(c), Role::Intermediate(i)) if i == c + 1 => {
            if *rule == Gate::Xor {
                role.insert(gate, Role::Z(i));
            }
            else if *rule == Gate::And {
                role.insert(gate, Role::CarryPart2(i));
            }
            else {
                return Err((gate, left_role, right_role));
            }
        }
        _ => {
                return Err((gate, left_role, right_role));
        }
    };
    Ok(())
}

// Finished in 2:53:47
fn part2(input: &str) -> String {
    let (_, rules) = input.split_once("\n\n").unwrap();
    let mut rules = rules
        .lines()
        .map(|line| {
            let terms = line.split(" ").collect::<Vec<&str>>();
            (
                terms[4],
                (
                    terms[0],
                    match terms[1] {
                        "XOR" => Gate::Xor,
                        "OR" => Gate::Or,
                        "AND" => Gate::And,
                        _ => panic!(""),
                    },
                    terms[2],
                ),
            )
        })
        .collect::<HashMap<&str, (&str, Gate, &str)>>();
        
    let mut swaps: Vec<&str> = vec![];
    loop {
        let mut roles = BTreeMap::new();
        let mut i = 0;
        let out = loop {
            let name = format!("z{i:<02}");
            if rules.get(name.as_str()).is_none() {
                break Ok(());
            }
            let out = rename_engine(
                rules.get_key_value(name.as_str()).unwrap().0,
                &mut roles,
                &rules
            );
            if out.is_err() {
                break out;
            }
            i += 1;
        };
        rules.iter().for_each(|(name, _)| {
            let _ = rename_engine(
                name,
                &mut roles,
                &rules
            );
        });
        if out.is_err() {
            let (left, rule, right) = rules.get(out.unwrap_err().0).unwrap();
            let left_role = roles.get(left).unwrap();
            let right_role = roles.get(right).unwrap();
            let mut problem = "";
            let mut should_be = Role::X(0);
            if *rule == Gate::Xor {
                if let (Role::Carry(x), other) | (other, Role::Carry(x)) = (left_role, right_role) {
                    problem = if other == left_role {left} else {right};
                    should_be = Role::Intermediate(x+1);
                }
                else if let (Role::Intermediate(x), other) | (other, Role::Intermediate(x)) = (left_role, right_role) {
                    problem = if other == left_role {left} else {right};
                    should_be = Role::Carry(x-1);
                }
                else {
                    panic!();
                }
            }
            else if *rule == Gate::And {
                if let (Role::Carry(x), other) | (other, Role::Carry(x)) = (left_role, right_role) {
                    problem = if other == left_role {left} else {right};
                    should_be = Role::Intermediate(x+1);
                }
                else if let (Role::Intermediate(x), other) | (other, Role::Intermediate(x)) = (left_role, right_role) {
                    problem = if other == left_role {left} else {right};
                    should_be = Role::Carry(x-1);
                }
                else {
                    panic!();
                }
            }
            else if *rule == Gate::Or {
                if let (Role::CarryPart1(x), other) | (other, Role::CarryPart1(x)) = (left_role, right_role) {
                    problem = if other == left_role {left} else {right};
                    should_be = Role::CarryPart2(*x);
                }
                else if let (Role::CarryPart2(x), other) | (other, Role::CarryPart2(x)) = (left_role, right_role) {
                    problem = if other == left_role {left} else {right};
                    should_be = Role::CarryPart1(*x);
                }
                else {
                    panic!();
                }
            }
            swaps.push(problem);
            let other_problem = *roles.iter().find(|(_, role)| **role == should_be).unwrap().0;
            swaps.push(other_problem);
            let one = rules.get(&problem).unwrap().clone();
            *rules.get_mut(&problem).unwrap() = *rules.get(&other_problem).unwrap();
            *rules.get_mut(&other_problem).unwrap() = one;
            continue
        }
        break;
    }
    swaps.sort();
    swaps.join(",")
}

#[cfg(test)]
#[test]
fn test_example() {
    let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
    assert_eq!(
        part1(input),
        usize::from_str_radix("0011111101000", 2).unwrap()
    );
    assert_eq!(part2(input), String::from(""));
}
