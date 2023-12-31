fn main() {
    let input: &str = include_str!("09.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> i64 {
    let mut sum = 0i64;
    for line in input.lines() {
        let mut history: Vec<Vec<i64>> =
            vec![line.split(" ").map(|x| x.parse().unwrap()).collect()];
        loop {
            let last_line: &Vec<i64> = history.last().unwrap();
            let mut next_line: Vec<i64> = vec![];
            for i in 0..(history.last().unwrap().len() - 1) {
                next_line.push(last_line[i + 1] - last_line[i]);
            }
            if next_line.iter().all(|x| *x == 0) {
                sum += history.iter().map(|x| x.last().unwrap()).sum::<i64>();
                break;
            }
            history.push(next_line);
        }
    }
    sum
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    assert_eq!(part1(input), 114);
}

fn part2(input: &str) -> i64 {
    let mut sum = 0i64;
    for line in input.lines() {
        let mut history: Vec<Vec<i64>> =
            vec![line.split(" ").map(|x| x.parse().unwrap()).collect()];
        loop {
            let last_line: &Vec<i64> = history.last().unwrap();
            let mut next_line: Vec<i64> = vec![];
            for i in 0..(history.last().unwrap().len() - 1) {
                next_line.push(last_line[i + 1] - last_line[i]);
            }
            if next_line.iter().all(|x| *x == 0) {
                history.reverse();
                sum += history.iter().fold(0i64, |a, b| b[0] - a);
                break;
            }
            history.push(next_line);
        }
    }
    sum
}

#[cfg(test)]
#[test]
fn test_part2() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    assert_eq!(part2(input), 2);
}
