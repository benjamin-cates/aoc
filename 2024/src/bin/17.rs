use std::collections::HashSet;

fn main() {
    let input: &str = include_str!("../data/17.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} ({:?})", part1(input), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} ({:?})", part2(input), now.elapsed());
}

// Finished in 18:07
fn part1(input: &str) -> String {
    let program = input.find("Program: ").unwrap() + "Program: ".len();
    let program = input[program..]
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .chunks(2)
        .map(|c| (c[0], c[1]))
        .collect::<Vec<(usize, usize)>>();
    let regs = input
        .split_once("\n\n")
        .unwrap()
        .0
        .lines()
        .map(|line| {
            line[line.find(":").unwrap() + 1..]
                .trim()
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<usize>>();
    let mut output = vec![];
    let (mut a, mut b, mut c) = (regs[0], regs[1], regs[2]);
    let mut instruction_pointer = 0;
    while instruction_pointer < program.len() {
        let (inst, operand) = program[instruction_pointer];
        if inst == 0 {
            a = a >> [0, 1, 2, 3, a, b, c][operand];
        } else if inst == 1 {
            b ^= operand;
        } else if inst == 2 {
            b = [0, 1, 2, 3, a, b, c][operand] % 8;
        } else if inst == 3 {
            if a != 0 {
                instruction_pointer = operand;
                continue;
            }
        } else if inst == 4 {
            b ^= c;
        } else if inst == 5 {
            output.push([0, 1, 2, 3, a, b, c][operand] % 8);
        } else if inst == 6 {
            b = a >> [0, 1, 2, 3, a, b, c][operand];
        } else if inst == 7 {
            c = a >> [0, 1, 2, 3, a, b, c][operand];
        }
        instruction_pointer += 1;
    }
    output
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn prints<'a>(program: &'a Vec<(usize, usize)>, starting: usize) -> impl Iterator<Item=usize> + use<'a> {
    let (mut a, mut b, mut c) = (starting, 0, 0);
    let mut instruction_pointer = 0;
    std::iter::from_fn(move || {
        while instruction_pointer < program.len() {
            let (inst, operand) = program[instruction_pointer];
            if inst == 0 {
                a = a >> [0, 1, 2, 3, a, b, c][operand];
            } else if inst == 1 {
                b ^= operand;
            } else if inst == 2 {
                b = [0, 1, 2, 3, a, b, c][operand] % 8;
            } else if inst == 3 {
                if a != 0 {
                    instruction_pointer = operand;
                    continue;
                }
            } else if inst == 4 {
                b ^= c;
            } else if inst == 5 {
                let value = [0, 1, 2, 3, a, b, c][operand] % 8;
                instruction_pointer += 1;
                return Some(value);
            } else if inst == 6 {
                b = a >> [0, 1, 2, 3, a, b, c][operand];
            } else if inst == 7 {
                c = a >> [0, 1, 2, 3, a, b, c][operand];
            }
            instruction_pointer += 1;
        }
        return None;
    })
}

// Finished in 02:06:25
fn part2(input: &str) -> usize {
    let program = input.find("Program: ").unwrap() + "Program: ".len();
    let program_nums = input[program..]
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let program = program_nums
        .chunks(2)
        .map(|c| (c[0], c[1]))
        .collect::<Vec<(usize, usize)>>();
    // Program: 2,4,1,1,7,5,4,6,1,4,0,3,5,5,3,0
    // b = a % 8   (2,4)
    // b ^= 1      (1,1)
    // c  = a >> b (7,5)
    // b ^= c      (4,6)
    // b ^= a      (1,4)
    // a = a >> 3  (0,3)
    // print(b)    (5,5)
    // jne a 0     (3,0)
    // b = 1 ^ (a >> (a ^ 1 % 8))
    let mut possibilities: HashSet<usize> = (0..0b1111111).collect();
    for i in 1..program_nums.len() {
        let mut new_possibilities = HashSet::new();
        for possib in possibilities.iter().cloned() {
            for attempt in 0..=7 {
                let starter = possib | (attempt << (i * 3 + 4));
                let mut printer = prints(&program, starter);
                if program_nums.iter().cloned().take(i).all(|v| Some(v) == printer.next()) {
                    new_possibilities.insert(starter);
                }
            }
        }
        possibilities = new_possibilities;
    }
    for possib in possibilities.iter().cloned() {
        let mut printer = prints(&program, possib);
        if program_nums.iter().cloned().all(|v| Some(v) == printer.next()) {
             if prints(&program, possib).count() == program_nums.len() {
                return possib;
            }
        }
    }
    0
}

// Program in the example is
// b = (a % 8 xor 1) xor (a >> 1) xor 4
// a = a >> 3
// print(b % 8)
// goto start if a != 0

// Therefore a = ()
// 010 = (a % 8 xor 1) xor (a >> 1) xor 100
// 110 = (a % 8 xor 1) xor (a >> 1)
// 111 = (a % 8 xor a >> 1), a = -------0101

// Therefore a = --0
// 001 = (a % 8) xor (a >> 1)
// a = ----1110101

// Therefore a = --1
// 100 = (a % 8) xor (a >> 1)
// a = 01_1111_0101

// Therefore a = --0
// 100 = (a % 8) xor (a >> 1)
//

// 000011101101011000100001110100101111001001100010 = a xor 001001001001001001001001001001001... xor a >> 1 xor 100100100100100100100...

// Program: 2,4,1,1,7,5,4,6,1,4,0,3,5,5,3,0

//       101 110 000 000 110 101 001 100 011 001 000 010 100 100 001 111
// a = 0 110 100 000 000 100 110 001 000 010 001 111 100 111 000 001 010
// 0b110100000000100110001000010001111100111000001010

#[cfg(test)]
#[test]
fn test_example() {
    let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    assert_eq!(part1(input), "4,6,3,5,6,3,5,2,1,0");
    let input2 = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
    assert_eq!(part2(input2), 117440);
}
