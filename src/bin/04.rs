use std::collections::HashSet;
fn main() {
    let input: &str = include_str!("04.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn get_list(input: &str) -> Vec<u32> {
    input
        .split(" ")
        .filter(|x| x.len() != 0)
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_matching_in_card(line: &str) -> usize {
    let mut sections = line.split(": ").skip(1).next().unwrap().split(" | ");
    let winning_nums = get_list(sections.next().unwrap())
        .into_iter()
        .collect::<HashSet<u32>>();
    let cards = get_list(sections.next().unwrap());
    let mut points = 0;
    for card in cards {
        if winning_nums.contains(&card) {
            points += 1;
        }
    }
    points
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let count = get_matching_in_card(line);
            if count == 0 {
                0
            } else {
                1 << (count - 1)
            }
        })
        .sum()
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(part1(input), 13);
}

fn part2(input: &str) -> usize {
    let line_count = input.lines().count();
    let mut lines = input.lines();
    let mut card_counts = vec![1; line_count];
    for i in 0..line_count {
        let matching_nums = get_matching_in_card(lines.next().unwrap());
        for j in 1..=matching_nums {
            if i + j < line_count {
                card_counts[i + j] += card_counts[i];
            }
        }
    }
    card_counts.iter().sum()
}

#[cfg(test)]
#[test]
fn test_part2() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(part2(input), 30);
}
