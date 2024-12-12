fn main() {
    let input: &str = include_str!("../data/09.txt");
    let now = std::time::Instant::now();
    println!("Part 1: {} ({:?})", part1(input), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {} ({:?})", part2(input), now.elapsed());
}

// 6469046261078
// Finished in 21:51
fn part1(input: &str) -> usize {
    let mut indexes: Vec<i64> = vec![];
    format!("{}0", input)
        .as_bytes()
        .chunks(2)
        .enumerate()
        .for_each(|(chunk_id, a)| {
            indexes.extend(
                std::iter::repeat_n(chunk_id as i64, (a[0] - b'0') as usize)
                    .chain(std::iter::repeat_n(-1, (a[1] - b'0') as usize)),
            );
        });
    for i in 0.. {
        if i >= indexes.len() {
            break;
        }
        if indexes[i] == -1 {
            while indexes.last().unwrap() == &-1 {
                indexes.pop().unwrap();
            }
            indexes[i] = indexes.pop().unwrap()
        }
    }
    indexes
        .iter()
        .enumerate()
        .map(|(i, v)| i * (*v as usize))
        .sum::<usize>()
}

// 48:55
fn part2(input: &str) -> usize {
    let mut chunks: Vec<(usize, Option<usize>)> = vec![];
    format!("{}0", input)
        .as_bytes()
        .chunks(2)
        .enumerate()
        .for_each(|(chunk_id, a)| {
            chunks.push(((a[0] - b'0') as usize, Some(chunk_id)));
            chunks.push(((a[1] - b'0') as usize, None));
        });
    let mut i = chunks.len() - 1;
    while i != 0 {
        if let (size, Some(_)) = chunks[i] {
            if let Some(new_pos) = chunks
                .iter()
                .take(i)
                .position(|v| v.1 == None && v.0 >= size)
            {
                let free_size = chunks[new_pos].0;
                chunks[new_pos] = chunks[i];
                chunks[i].1 = None;
                chunks.insert(new_pos + 1, (free_size - size, None));
                continue;
            }
        }
        i -= 1;
    }
    chunks
        .iter()
        .map(|v| std::iter::repeat_n(v.1.unwrap_or(0), v.0))
        .flatten()
        .enumerate()
        .map(|(i, v)| i * (v as usize))
        .sum::<usize>()
}

#[cfg(test)]
#[test]
fn test_example() {
    let input = "2333133121414131402";
    assert_eq!(part1(input), 1928);
    assert_eq!(part2(input), 2858);
}
