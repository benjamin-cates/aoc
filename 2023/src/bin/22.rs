use aoc23::traverse::*;

fn main() {
    let input: &str = include_str!("22.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}
// part 1: 436 too high

#[derive(Eq, PartialEq, Clone, Debug)]
struct Brick {
    z_start: i32,
    z_end: i32,
    // Smaller corner
    cor_max: Point,
    // Larger corner
    cor_min: Point,
    can_disintigrate: bool,
}

impl Brick {
    fn new(x1: i32, x2: i32, y1: i32, y2: i32, z1: i32, z2: i32) -> Self {
        Brick {
            z_start: z1.min(z2),
            z_end: z1.max(z2),
            cor_max: Point {
                x: x1.max(x2),
                y: y1.max(y2),
            },
            cor_min: Point {
                x: x1.min(x2),
                y: y1.min(y2),
            },
            can_disintigrate: true,
        }
    }
    // Assuming self is above other
    fn vert_dist(&self, other: &Brick) -> Option<i32> {
        if self.z_end <= other.z_start {
            return None;
        }
        if self.cor_max.x < other.cor_min.x || other.cor_max.x < self.cor_min.x {
            return None;
        }
        if self.cor_max.y < other.cor_min.y || other.cor_max.y < self.cor_min.y {
            return None;
        }
        return Some(self.z_start - other.z_end - 1);
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.z_start.cmp(&other.z_start))
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

fn parse_bricks(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let p1: Vec<i32> = line
                .split("~")
                .nth(0)
                .unwrap()
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect();
            let p2: Vec<i32> = line
                .split("~")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect();
            Brick::new(p1[0], p2[0], p1[1], p2[1], p1[2], p2[2])
        })
        .collect()
}

fn settle_bricks(bricks: &mut Vec<Brick>) -> usize {
    let mut settle_count = 0;
    for idx in 0..bricks.len() {
        // Get list of blocks below it and the falling distance to it
        let below_dists: Vec<(usize, i32)> = (0..idx)
            .filter_map(|idx2| {
                bricks[idx]
                    .vert_dist(&bricks[idx2])
                    .map(|dist| (idx2, dist))
            })
            .collect();
        // Find the most it can fall without passing through a block
        match below_dists.iter().map(|(_idx2, dist)| dist).min() {
            Some(min) => {
                // Get bricks it would be resting on
                let resting: Vec<(usize, i32)> = below_dists
                    .iter()
                    .cloned()
                    .filter(|(_idx2, dist)| *dist == *min)
                    .collect();
                // If it would be resting on exactly one, the one below it cannot disintigrate
                if resting.len() == 1 {
                    bricks[resting[0].0].can_disintigrate = false;
                }
                if *min != 0 {
                    settle_count += 1;
                }
                bricks[idx].z_start -= min;
                bricks[idx].z_end -= min;
            }
            None => {
                if bricks[idx].z_start != 0 {
                    settle_count += 1;
                }
                bricks[idx].z_end -= bricks[idx].z_start;
                bricks[idx].z_start = 0;
            }
        };
    }
    settle_count
}

fn part1(input: &str) -> usize {
    let mut bricks = parse_bricks(input);
    // Sort by z height
    bricks.sort();
    settle_bricks(&mut bricks);
    bricks.iter().filter(|brick| brick.can_disintigrate).count()
}

fn part2(input: &str) -> usize {
    let mut bricks = parse_bricks(input);
    bricks.sort();
    settle_bricks(&mut bricks);
    // Okay so the ideal way to do this would be to create a more complicated settling function,
    // but the input is so short that it runs in less than 10 secs on release so I'm not gonna bother
    let mut count = 0;
    // Count number of fallen bricks for non-disintigratable blocks
    for idx in 0..bricks.len() {
        if !bricks[idx].can_disintigrate {
            let mut copy = bricks.clone();
            copy.remove(idx);
            count += settle_bricks(&mut copy);
        }
    }
    count
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    assert_eq!(part1(input), 5);
    assert_eq!(part2(input), 7);
}
