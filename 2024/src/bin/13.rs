fn main() {
    let input: &str = include_str!("../data/13.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} ({:?})", part1(input), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} ({:?})", part2(input), now.elapsed());
}
#[derive(Debug, Clone, Copy)]
struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    t_x: i64,
    t_y: i64,
}

impl Machine {
    fn parse(lines: &[&str]) -> Machine {
        let (a_x, a_y) = lines[0]
            .trim_matches(|c: char| !c.is_numeric())
            .split_once(", Y+")
            .unwrap();
        let (b_x, b_y) = lines[1]
            .trim_matches(|c: char| !c.is_numeric())
            .split_once(", Y+")
            .unwrap();
        let (t_x, t_y) = lines[2]
            .trim_matches(|c: char| !c.is_numeric())
            .split_once(", Y=")
            .unwrap();
        Machine {
            a_x: a_x.parse().unwrap(),
            a_y: a_y.parse().unwrap(),
            b_x: b_x.parse().unwrap(),
            b_y: b_y.parse().unwrap(),
            t_x: t_x.parse().unwrap(),
            t_y: t_y.parse().unwrap(),
        }
    }
    fn needed_tokens(&self) -> Option<usize> {
        (0..100)
            .filter_map(|num_b| {
                let rem_x = self.t_x - self.b_x * num_b;
                let rem_y = self.t_y - self.b_y * num_b;
                if rem_x % self.a_x == 0 && rem_y % self.a_y == 0 {
                    if rem_x / self.a_x == rem_y / self.a_y {
                        return Some((rem_x / self.a_x * 3 + num_b) as usize);
                    }
                }
                return None;
            })
            .min()
    }
    fn needed_tokens_part2(&self) -> Option<usize> {
        let a = self.t_x * self.b_y - self.b_x * self.t_y;
        let b = self.a_x * self.t_y - self.t_x * self.a_y;
        let det = self.a_x * self.b_y - self.a_y * self.b_x;
        if (a % det != 0) || (b % det != 0) {
            return None;
        }
        if a / det >= 0 && b / det >= 0 {
            return Some((a / det * 3 + b / det) as usize);
        } else {
            return None;
        }
    }
}

// Finished in 15:30
fn part1(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut out = 0;
    for info in lines.chunks(4) {
        let machine = Machine::parse(info);
        out += machine.needed_tokens().unwrap_or(0);
    }
    out
}

// Finished in 1:06:53
fn part2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut out = 0;
    for info in lines.chunks(4) {
        let mut machine = Machine::parse(info);
        machine.t_x += 10000000000000;
        machine.t_y += 10000000000000;
        out += machine.needed_tokens_part2().unwrap_or(0);
    }
    out
}

#[cfg(test)]
#[test]
fn test_example() {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
    assert_eq!(part1(input), 480);
    assert_eq!(part2(input), 875318608908);
}
