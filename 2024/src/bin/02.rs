fn main() {
    let input: &str = include_str!("../data/02.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn is_safe(nums: &Vec<i32>) -> bool {
    // Check if there are no "hills"
    for i in 0..nums.len() - 1 {
        if i != 0 {
            if nums[i] > nums[i + 1] && nums[i] > nums[i - 1] {
                return false;
            }
            if nums[i] < nums[i + 1] && nums[i] < nums[i - 1] {
                return false;
            }
        }
        if nums[i].abs_diff(nums[i + 1]) < 1 || nums[i].abs_diff(nums[i + 1]) > 3 {
            return false;
        }
    }
    return true;
}

// Solved in 6:00
fn part1(input: &str) -> usize {
    let mut count = 0;
    for line in input.lines() {
        let nums = line
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if is_safe(&nums) {
            count += 1;
        }
    }
    count
}


fn is_good_inc(nums: &Vec<i32>) -> bool {
    let mut differences = vec![];
    for i in 0..nums.len() - 1 {
        differences.push(nums[i+1] - nums[i]);
    }
    return differences.iter().filter(|dif| *dif >= &1 && *dif <= &3).count() == differences.len();
}

fn dampened_safe_nums(nums: &Vec<i32>) -> bool {
    let mut differences = vec![];
    for i in 0..nums.len() - 1 {
        differences.push(nums[i+1] - nums[i]);
    }
    if is_good_inc(nums) {
        return true;
    }
    let naughty = differences.iter().position(|v| *v < 1 || *v > 3).unwrap();
    let removed = nums.iter().enumerate().filter(|a| a.0 != naughty).map(|a| a.1).cloned().collect::<Vec<_>>();
    if is_good_inc(&removed) {
        return true;
    }
    let removed = nums.iter().enumerate().filter(|a| a.0 != naughty + 1).map(|a| a.1).cloned().collect::<Vec<_>>();
    if is_good_inc(&removed) {
        return true;
    }
    return false;
}

fn dampened_safe(line: &str) -> bool {
    let mut nums = line
        .split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    if dampened_safe_nums(&nums) {
        return true;
    }
    nums.reverse();
    if dampened_safe_nums(&nums) {
        return true;
    }
    return false;
}


// Took 1:18:00
fn part2(input: &str) -> usize {
    let mut count = 0;
    for line in input.lines() {
        if dampened_safe(line) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    assert_eq!(part1(input), 2);
    assert_eq!(part2(input), 4);
}
