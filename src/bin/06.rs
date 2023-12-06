fn main() {
    let input: &str = include_str!("06.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn num_wins(time: u64, dist: u64) -> u64 {
    // i * (time - i) > dist
    // - i^2 + time*i - dist > 0
    // i = (-time ± sqrt(time^2 - 4dist)) / 2
    let t = time as f32;
    let d = dist as f32;
    let i0 = ((-t + (t * t - 4. * d).sqrt()) / -2. + 0.00001).ceil();
    let i1 = ((-t - (t * t - 4. * d).sqrt()) / -2. - 0.00001).ceil();
    (i1 - i0) as u64
}

fn part1(input: &str) -> u64 {
    let time_iter = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap());
    let dist_iter = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap());
    time_iter
        .zip(dist_iter)
        .map(|(time, dist)| num_wins(time, dist))
        .product()
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    assert_eq!(part1(input), 288);
}

fn part2(input: &str) -> u64 {
    let time: u64 = input
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();
    let dist: u64 = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();
    num_wins(time, dist)
}

#[cfg(test)]
#[test]
fn test_part2() {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    assert_eq!(part2(input), 71503);
}
