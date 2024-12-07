fn main() {
    let input: &str = &include_str!("../data/07.txt");
    let now = std::time::Instant::now();
    for _ in 0..10000 {
        part1(input);
    }
    println!("Time: {:?}", now.elapsed() / 10000);
    let now = std::time::Instant::now();
    for _ in 0..10000 {
        part2(input);
    }
    println!("Time: {:?}", now.elapsed() / 10000);
}
fn works(nums: &[i64], target: i64, part2: bool) -> bool {
    if nums.len() == 0 {
        return target == 0;
    }
    if target <= 0 {
        return false;
    }
    let last = nums.last().unwrap();

    // Multiplication case
    if target % last == 0 {
        if works(&nums[0..nums.len() - 1], target / last, part2) {
            return true;
        }
    }

    // Concatenation case
    if part2 {
        let concat_digits = (10i64).pow(last.ilog(10) + 1);
        if (target - last) % concat_digits == 0 {
            if works(&nums[0..nums.len() - 1], target / concat_digits, part2) {
                return true;
            }
        }
    }

    // Addition case
    return works(&nums[0..nums.len() - 1], target - last, part2);
}

fn part1(input: &str) -> usize {
    let mut count = 0;
    for line in input.lines() {
        let (ans, right) = line.split_once(": ").unwrap();
        let ans = ans.parse::<i64>().unwrap();
        let nums = right
            .split(" ")
            .map(|v| v.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        if works(nums.as_slice(), ans, false) {
            count += ans;
        }
    }
    count as usize
}

fn part2(input: &str) -> usize {
    let mut count = 0;
    for line in input.lines() {
        let (ans, right) = line.split_once(": ").unwrap();
        let ans = ans.parse::<i64>().unwrap();
        let nums = right
            .split(" ")
            .map(|v| v.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        if works(nums.as_slice(), ans, true) {
            count += ans;
        }
    }
    count as usize
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    assert_eq!(part1(input), 3749);
    assert_eq!(part2(input), 11387);
}
