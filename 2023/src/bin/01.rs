fn main() {
    let input: &str = include_str!("01.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let nums: Vec<u8> = line
                .bytes()
                .filter(|x| (&b'0' <= x && x <= &b'9')) // Filter out non digits
                .map(|x| x - b'0') // Convert to integers
                .collect(); // Collect as vector
            (nums[0] * 10 + nums[nums.len() - 1]) as usize // First * 10 + last
        })
        .sum() // Sum all lines
}

#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1("123"), 13);
    assert_eq!(part1("e1o0e"), 10);
    assert_eq!(part1("e1oo"), 11);
    assert_eq!(part1("two01"), 01);
    assert_eq!(part1("fourfivesix1"), 11);
    assert_eq!(part1("000000"), 00);
    assert_eq!(part1("9998"), 98);
    assert_eq!(part1("1"), 11);
    assert_eq!(part1("604"), 64);
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|x| {
            x.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "4")
                .replace("five", "5e")
                .replace("six", "6")
                .replace("seven", "7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
        })
        .map(|line| {
            let nums: Vec<u8> = line
                .bytes()
                .filter(|x| (&b'0' <= x && x <= &b'9')) // Filter out non digits
                .map(|x| x - b'0') // Convert to integers
                .collect(); // Collect as vector
            (nums[0] * 10 + nums[nums.len() - 1]) as usize // First * 10 + last
        })
        .sum() // Sum all lines
}

#[cfg(test)]
#[test]
fn test_part2() {
    assert_eq!(part2("oneight2"), 12);
    assert_eq!(part2("oneightwo"), 12);
    assert_eq!(part2("eightwo"), 82);
    assert_eq!(part2("713"), 73);
    assert_eq!(part2("sevenine"), 79);
    assert_eq!(part2("nineight"), 98);
    assert_eq!(part2("fiveight"), 58);
    assert_eq!(part2("oneight"), 18);
    assert_eq!(part2("eighthree"), 83);
    assert_eq!(part2("eight2"), 82);
    assert_eq!(part2("2three"), 23);
    assert_eq!(part2("11"), 11);
    assert_eq!(part2("two8eightwo"), 22);
}
