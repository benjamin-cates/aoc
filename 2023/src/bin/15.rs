fn main() {
    let input: &str = include_str!("../data/15.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn hash(input: &str) -> usize {
    input
        .bytes()
        .filter(|x| *x != b'\n')
        .fold(0, |cur, char| ((cur + char as usize) * 17) % 256)
}

fn part1(input: &str) -> usize {
    input.split(",").map(hash).sum()
}

fn part2(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
    for x in input.split(",") {
        let control_pos = x.bytes().position(|ch| ch == b'=' || ch == b'-').unwrap();
        let label = &x[0..control_pos];
        let hash = hash(label);
        let pos = boxes[hash].iter().position(|y| y.0 == label);
        if x.bytes().nth(control_pos) == Some(b'=') {
            let value = x[(control_pos + 1)..(control_pos + 2)].parse().unwrap();
            if let Some(pos) = pos {
                boxes[hash][pos].1 = value;
            } else {
                boxes[hash].push((label, value));
            }
        } else if pos.is_some() {
            boxes[hash].remove(pos.unwrap());
        }
    }
    boxes
        .iter()
        .enumerate()
        .map(|(box_id, box_contents)| -> usize {
            box_contents
                .iter()
                .enumerate()
                .map(|(id, (_, focal))| (id + 1) * focal)
                .sum::<usize>()
                * (box_id + 1)
        })
        .sum()
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(part1(input), 1320);
    assert_eq!(part2(input), 145);
}
