fn main() {
    let input: &str = include_str!("../data/02.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn is_good_inc(nums: &Vec<i32>) -> bool {
    // Create an iterator over the differences and see if the count of valid differences is equal to the total number of differences
    (0..nums.len() - 1)
        .map(|i| nums[i + 1] - nums[i])
        .filter(|v| *v >= 1 && *v <= 3).count() == nums.len() - 1
}

// Solved in 6:00
fn part1(input: &str) -> usize {
    let mut count = 0;
    for line in input.lines() {
        let mut nums = line
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if is_good_inc(&nums) {
            count += 1;
            continue;
        }
        nums.reverse();
        if is_good_inc(&nums) {
            count += 1;
        }
    }
    count
}

fn dampened_good_inc(nums: &Vec<i32>) -> bool {
    if is_good_inc(nums) {
        return true;
    }
    // Find the offending index
    let naughty = (0..nums.len() - 1)
        .map(|i| nums[i + 1] - nums[i])
        .position(|v| v < 1 || v > 3)
        .unwrap();

    // Try removing that index
    let mut removed = nums.clone();
    removed.remove(naughty);
    if is_good_inc(&removed) {
        return true;
    }

    // Try removing the next element
    let mut removed = nums.clone();
    removed.remove(naughty + 1);
    if is_good_inc(&removed) {
        return true;
    }

    return false;
}

// Took 1:18:00
fn part2(input: &str) -> usize {
    let mut count = 0;
    for line in input.lines() {
        let mut nums = line
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        // See if it's mostly increasing safe
        if dampened_good_inc(&nums) {
            count += 1;
            continue;
        }
        // Else, see if it's mostly decreasing safe
        nums.reverse();
        if dampened_good_inc(&nums) {
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
