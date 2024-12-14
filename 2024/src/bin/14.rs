use std::str::FromStr;

use aoc24::Point;

fn main() {
    let input: &str = include_str!("../data/14.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} ({:?})", part1(input, 101, 103), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} ({:?})", part2(input, 101, 103), now.elapsed());
}

#[derive(Clone, Copy, Debug)]
struct Robot {
    pos: Point,
    vel: Point,
}

impl FromStr for Robot {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut pos, mut vel) = s.split_once(" ").unwrap();
        pos = &pos[2..];
        vel = &vel[2..];
        let mut point = pos.split(",").filter_map(|v| v.parse::<i32>().ok());
        let mut vel = vel.split(",").filter_map(|v| v.parse::<i32>().ok());
        Ok(Robot {
            pos: (point.next().ok_or(())?, point.next().ok_or(())?).into(),
            vel: (vel.next().ok_or(())?, vel.next().ok_or(())?).into(),
        })
    }
}

// Finished in 13:19
fn part1(input: &str, width: i32, height: i32) -> usize {
    let mut robots: Vec<Robot> = input.lines().map(|v| v.parse().unwrap()).collect();
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.pos = robot.pos + robot.vel;
            robot.pos.x = (robot.pos.x + width) % width;
            robot.pos.y = (robot.pos.y + height) % height;
        }
    }
    let mut quads = [0; 5];
    for robot in robots.iter() {
        use std::cmp::Ordering::*;
        let quadrant: usize = match (
            robot.pos.y.cmp(&(height / 2)),
            robot.pos.x.cmp(&(width / 2)),
        ) {
            (Less, Less) => 1,
            (Less, Greater) => 2,
            (Greater, Less) => 3,
            (Greater, Greater) => 4,
            _ => 0,
        };
        quads[quadrant] += 1;
    }
    quads[1] * quads[2] * quads[3] * quads[4]
}

// Finished in 39:54
fn part2(input: &str, width: i32, height: i32) -> usize {
    let mut robots: Vec<Robot> = input.lines().map(|v| v.parse().unwrap()).collect();
    for i in 1.. {
        // Count the number in each column
        let mut cols = [0; 102];
        // Count the number in each row
        let mut rows = [0; 103];
        for robot in robots.iter_mut() {
            robot.pos = robot.pos + robot.vel;
            robot.pos.x = (robot.pos.x + width) % width;
            robot.pos.y = (robot.pos.y + height) % height;
            cols[robot.pos.x as usize] += 1;
            rows[robot.pos.y as usize] += 1;
        }
        // If there are two rows with at least 30 and two columns with at least 30, then there is probabbly a square
        if cols.iter().filter(|v| **v >= 30).count() >= 2
            && rows.iter().filter(|v| **v >= 30).count() >= 2
        {
            return i;
        }
    }
    return 0;
}

#[cfg(test)]
#[test]
fn test_example() {
    let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    assert_eq!(part1(input, 11, 7), 12);
    // Part 2 does not have a test input
    //assert_eq!(part2(input, 11, 7), 0);
}
