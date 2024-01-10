fn main() {
    let input: &str = include_str!("template.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "";
    assert_eq!(part1(input), 0);
    assert_eq!(part2(input), 0);
}
