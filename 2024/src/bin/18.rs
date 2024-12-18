use aoc24::{CharGrid, Direction, Point, StaticGraph};

fn main() {
    let input: &str = include_str!("../data/18.txt");
    let now = std::time::Instant::now();
    println!(
        "Part 1: {} ({:?})",
        part1(input, 71, 1024).unwrap(),
        now.elapsed()
    );
    let now = std::time::Instant::now();
    println!("Part 2: {} ({:?})", part2(input, 71, 1024), now.elapsed());
}

// Finished in 9:20
fn part1(input: &str, size: usize, num_take: usize) -> Option<usize> {
    let mut grid = CharGrid {
        grid: vec![vec!['.'; size]; size],
    };
    let mut edges: Vec<(Point, Point)> = vec![];
    for line in input.lines().take(num_take) {
        let (x, y) = line.split_once(',').unwrap();
        let point: Point = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()).into();
        grid[point] = '#';
    }
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let point: Point = (x, y).into();
            for dir in Direction::iter_all() {
                if grid[point] == '.' && grid.get(point + dir.step()) == Some('.') {
                    edges.push((point, point + dir.step()));
                }
            }
        }
    }
    let graph = StaticGraph::new().add_undirected_edges(edges);
    graph
        .dijkstras(&(0, 0).into(), |p| *p == (size - 1, size - 1).into())
        .map(|path| path.len() - 1)
}

// Finished in 13:30
fn part2(input: &str, size: usize, num_start: usize) -> String {
    // Binary search to find the first element that fails
    let mut min = num_start;
    let mut max = input.lines().count();
    let idx = loop {
        let mid = (min + max) / 2;
        if mid == max {
            break mid;
        }
        if part1(input, size, mid).is_none() {
            max = mid;
        } else {
            min = mid + 1;
        }
    };
    return input.lines().nth(idx - 1).unwrap().to_string();
}

#[cfg(test)]
#[test]
fn test_example() {
    let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    assert_eq!(part1(input, 7, 12), Some(22));
    assert_eq!(part2(input, 7, 12), "6,1".to_string());
}
