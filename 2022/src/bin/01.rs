fn main() {
    let input = include_str!("../data/01.txt");
    println!("Answer to part 1: {}", part1(input));
    println!("Answer to part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut cur_sum: usize = 0;
    let mut max = 0;
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            if cur_sum > max {
                max = cur_sum;
            }
            cur_sum = 0;
        } else {
            cur_sum += line.parse::<usize>().unwrap();
        }
    }
    if cur_sum > max {
        max = cur_sum;
    }
    max
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    assert_eq!(part1(input), 24000);
    let input2 = "412451";
    assert_eq!(part1(input2), 412451);
}

use std::collections::BinaryHeap;
fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let mut cur_sum: usize = 0;
    let mut cals = BinaryHeap::new();
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            cals.push(cur_sum);
            cur_sum = 0;
        } else {
            cur_sum += line.parse::<usize>().unwrap();
        }
    }
    cals.push(cur_sum);
    cals.pop().unwrap() + cals.pop().unwrap() + cals.pop().unwrap()
}

#[cfg(test)]
#[test]
fn test_part2() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    assert_eq!(part2(input), 45000);
}
