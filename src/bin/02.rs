fn main() {
    let input = include_str!("02.txt");
    println!("Answer to part 1: {}", part1(input));
    println!("Answer to part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let maxes: std::collections::HashMap<&str, usize> =
                [("red", 12), ("green", 13), ("blue", 14)]
                    .into_iter()
                    .collect();
            let mut game_iter = line[5..].split(": ");
            let id = game_iter.next().unwrap().parse().unwrap();
            for set in game_iter.next().unwrap().split("; ") {
                for item in set.split(", ") {
                    let mut iter = item.split(" ");
                    let num: usize = iter.next().unwrap().parse().unwrap();
                    let col: &str = iter.next().unwrap();
                    if num > maxes[col] {
                        return 0;
                    }
                }
            }
            return id;
        })
        .sum()
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(part1(input), 8);
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut mins: std::collections::HashMap<&str, usize> =
                [("red", 0), ("green", 0), ("blue", 0)]
                    .into_iter()
                    .collect();
            let sets = line.split(": ").skip(1).next().unwrap().split("; ");
            for set in sets {
                let items = set.split(", ");
                for item in items {
                    let mut iter = item.split(" ");
                    let num: usize = iter.next().unwrap().parse().unwrap();
                    let col: &str = iter.next().unwrap();
                    mins.insert(col, mins[col].max(num));
                }
            }
            mins["red"] * mins["green"] * mins["blue"]
        })
        .sum()
}

#[cfg(test)]
#[test]
fn test_part2() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(part2(input), 2286);
}
