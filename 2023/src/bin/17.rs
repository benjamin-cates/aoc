use aoc23::traverse::Dir::*;
use aoc23::traverse::*;
use derivative::Derivative;
use std::collections::BinaryHeap;
use std::collections::HashSet;

fn main() {
    let input: &str = include_str!("../data/17.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}
#[derive(Derivative, Clone, Eq)]
#[derivative(PartialEq, Hash)]
struct Node {
    pos: Point,
    dir: Dir,
    num_in_dir: u8,
    #[derivative(Hash = "ignore")]
    #[derivative(PartialEq = "ignore")]
    heat_loss: usize,
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.heat_loss.cmp(&self.heat_loss))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn part1(input: &str) -> usize {
    crucible_traverse(input, 0, 3).unwrap()
}

fn part2(input: &str) -> usize {
    crucible_traverse(input, 4, 10).unwrap()
}

fn crucible_traverse(input: &str, min_len: u8, max_len: u8) -> Option<usize> {
    // Dijkstra's on an graph where the node is stored as (position, direction, num_in_dir)
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|x| {
            x.split("")
                .filter(|ch| ch.len() == 1)
                .map(|ch| ch.parse().unwrap())
                .collect()
        })
        .collect();
    let mut visited_set: HashSet<Node> = HashSet::new();
    let mut to_visit_set: BinaryHeap<Node> = BinaryHeap::new();
    to_visit_set.push(Node {
        pos: (1, 0).into(),
        dir: East,
        num_in_dir: 1,
        heat_loss: grid[0][1],
    });
    to_visit_set.push(Node {
        pos: (0, 1).into(),
        dir: South,
        num_in_dir: 1,
        heat_loss: grid[1][0],
    });
    while !to_visit_set.is_empty() {
        let visiting = to_visit_set.pop().unwrap();
        if visited_set.get(&visiting).is_some() {
            continue;
        }
        for new_dir in [North, South, East, West] {
            if new_dir == visiting.dir.opposite() {
                continue;
            }
            let new_pos = visiting.pos + new_dir.step();
            if new_pos.x < 0
                || new_pos.y < 0
                || new_pos.x >= grid[0].len() as i32
                || new_pos.y >= grid.len() as i32
            {
                continue;
            }
            let new_node = Node {
                pos: new_pos,
                dir: new_dir,
                num_in_dir: if new_dir == visiting.dir {
                    if visiting.num_in_dir == max_len {
                        continue;
                    } else {
                        visiting.num_in_dir + 1
                    }
                } else {
                    if visiting.num_in_dir < min_len {
                        continue;
                    } else {
                        1
                    }
                },
                heat_loss: visiting.heat_loss + grid[new_pos.y as usize][new_pos.x as usize],
            };
            if new_pos.x as usize == grid[0].len() - 1
                && new_pos.y as usize == grid.len() - 1
                && new_node.num_in_dir >= min_len
            {
                return Some(new_node.heat_loss);
            }
            if visited_set.get(&new_node).is_some() {
                continue;
            }
            to_visit_set.push(new_node);
        }
        visited_set.insert(visiting);
    }
    None
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    assert_eq!(part1(input), 102);
    assert_eq!(part2(input), 94);
    assert_eq!(part1(include_str!("../data/17.txt")), 1110);
    let input2 = "111111111111
999999999991
999999999991
999999999991
999999999991";
    assert_eq!(part2(input2), 71);
}
