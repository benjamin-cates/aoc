use std::{cmp::Ordering, collections::HashMap};
fn main() {
    let input: &str = include_str!("../data/07.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

struct Bid {
    bid: u64,
    card_value: u64,
    hand_type: u64,
}

impl PartialEq for Bid {
    fn eq(&self, other: &Bid) -> bool {
        return self.hand_type == other.hand_type && self.card_value == other.card_value;
    }
}
impl Eq for Bid {}

impl PartialOrd for Bid {
    fn partial_cmp(&self, other: &Bid) -> Option<Ordering> {
        //Order first by hand type, then by the values of the card hand
        Some(if self.hand_type < other.hand_type {
            Ordering::Less
        } else if self.hand_type > other.hand_type {
            Ordering::Greater
        } else if self.card_value < other.card_value {
            Ordering::Less
        } else if self.hand_type == other.hand_type && self.card_value == other.card_value {
            Ordering::Equal
        } else {
            Ordering::Greater
        })
    }
}

impl Ord for Bid {
    fn cmp(&self, other: &Bid) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn get_hand_type(num_matching_sets: Vec<u64>) -> u64 {
    // Five of a kind
    if num_matching_sets[5] == 1 {
        6
    }
    // Four of a kind
    else if num_matching_sets[4] == 1 {
        5
    }
    // Full house
    else if num_matching_sets[3] == 1 && num_matching_sets[2] == 1 {
        4
    }
    // Three of a kind
    else if num_matching_sets[3] == 1 {
        3
    }
    // Double pair
    else if num_matching_sets[2] == 2 {
        2
    }
    // Single pair
    else if num_matching_sets[2] == 1 {
        1
    }
    // High card
    else {
        0
    }
}

fn apply_jokers(num_matching_sets: &mut Vec<u64>, jokers: u64) {
    for _ in 0..jokers {
        // Find the matching set with the largest size and increment its size
        let idx_max = num_matching_sets
            .iter()
            .enumerate()
            .filter(|(_, a)| **a != 0)
            .last()
            .unwrap()
            .0;
        num_matching_sets[idx_max] -= 1;
        num_matching_sets[idx_max + 1] += 1;
    }
}

fn hand_to_bid_value(cards: &str, bid: u64, is_part_2: bool) -> Bid {
    let card_ordering: HashMap<char, u64> = vec![
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', if is_part_2 { 1 } else { 11 }),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]
    .into_iter()
    .collect();
    // Value of each card
    let card_values: Vec<u64> = cards
        .chars()
        .map(|x| *card_ordering.get(&x).unwrap())
        .collect();
    // Find out the value of the card based primarily on the value of the first
    // index, then by the value of the second index, etc...
    let cumm_card_value = card_values.iter().fold(0, |a, b| a * 15 + b);
    // Get quantity of each card type
    let mut of_each = vec![0u64; 15];
    for value in card_values.iter() {
        of_each[*value as usize] += 1;
    }
    // Jokers if in part 2
    let jokers = of_each[1];
    if is_part_2 {
        of_each[1] = 0;
    }
    // Get quantity of each size of matching set
    // Ex. 2 pair would be [_, 1, 2, 0, 0, 0]
    // Ex. 5 of a kind would be [_, 0, 0, 0, 0, 1]
    // Ex. full house would be [_, 0, 1, 1, 0, 0]
    let mut num_matching_sets: Vec<u64> = (0..=5)
        .map(|x| of_each.iter().filter(|y| **y == x).count() as u64)
        .collect::<Vec<u64>>();
    if is_part_2 {
        apply_jokers(&mut num_matching_sets, jokers);
    }
    Bid {
        bid,
        card_value: cumm_card_value,
        hand_type: get_hand_type(num_matching_sets),
    }
}

fn score_bids(mut bids: Vec<Bid>) -> u64 {
    bids.sort();
    bids.into_iter()
        .enumerate()
        .map(|(idx, bid)| (idx as u64 + 1) * bid.bid)
        .sum()
}

fn part1(input: &str) -> u64 {
    score_bids(
        input
            .lines()
            .map(|x| {
                hand_to_bid_value(
                    x.split_once(" ").unwrap().0,
                    x.split_once(" ").unwrap().1.parse().unwrap(),
                    false,
                )
            })
            .collect::<Vec<Bid>>(),
    )
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(part1(input), 6440);
}

fn part2(input: &str) -> u64 {
    score_bids(
        input
            .lines()
            .map(|x| {
                hand_to_bid_value(
                    x.split_once(" ").unwrap().0,
                    x.split_once(" ").unwrap().1.parse().unwrap(),
                    true,
                )
            })
            .collect::<Vec<Bid>>(),
    )
}

#[cfg(test)]
#[test]
fn test_part2() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(part2(input), 5905);
}
