use std::time::Instant;
fn main() {
    let input: &str = include_str!("../data/05.txt");
    let start = Instant::now();
    println!("Answer to part1: {}", part1(input));
    println!("Part 1 took: {:?}", start.elapsed());
    let start = Instant::now();
    println!("Answer to part2: {}", part2(input));
    println!("Part 2 took: {:?}", start.elapsed());
}

#[derive(Debug)]
struct Val {
    idx: usize,
    stage: usize,
}

fn get_min_location(input: &str, seeds_list: &mut Vec<Val>) -> usize {
    let mut lines = input.lines();
    // Discard meaningless lines
    lines.next().unwrap();
    lines.next().unwrap();
    for i in 0..7 {
        // Discard meaningless line
        lines.next().unwrap();
        while let Some(line) = lines.next() {
            if line.len() == 0 {
                break;
            }
            let nums = line
                .split(" ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            for seed in seeds_list.iter_mut() {
                if seed.stage != i {
                    continue;
                }
                if seed.idx >= nums[1] && seed.idx <= nums[1] + nums[2] - 1 {
                    seed.idx += nums[0];
                    seed.idx -= nums[1];
                    seed.stage += 1;
                }
            }
        }
        for seed in seeds_list.iter_mut() {
            if seed.stage == i {
                seed.stage += 1;
            }
        }
    }
    seeds_list.iter().map(|x| x.idx).min().unwrap()
}

fn part1(input: &str) -> usize {
    let mut seeds_list: Vec<Val> = input
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|x| Val {
            idx: x.parse().unwrap(),
            stage: 0,
        })
        .collect();
    get_min_location(input, &mut seeds_list)
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_eq!(part1(input), 35);
}

#[derive(Debug)]
struct ValPart2 {
    start: usize,
    len: usize,
    stage: usize,
}

fn get_min_location_2(input: &str, seeds_list: &mut Vec<ValPart2>) -> usize {
    let mut lines = input.lines();
    // Discard meaningless lines
    lines.next().unwrap();
    lines.next().unwrap();
    for i in 0..7 {
        // Discard meaningless line
        lines.next().unwrap();
        while let Some(line) = lines.next() {
            if line.len() == 0 {
                break;
            }
            let nums = line
                .split(" ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let mut j = 0;

            while let Some(seed) = seeds_list.get(j) {
                if seed.stage != i {
                    j += 1;
                    continue;
                }
                let dest_start = nums[0];
                let range_start = nums[1];
                let range_last = nums[1] + nums[2] - 1;
                let seed_start = seed.start;
                let seed_last = seed.start + seed.len - 1;
                let start_in_range: bool = seed_start >= range_start && seed_start <= range_last;
                let end_in_range: bool = seed_last >= range_start && seed_last <= range_last;
                // Check 4 different kinds of seed-translation overlaps
                if start_in_range && end_in_range {
                    seeds_list[j].start += dest_start;
                    seeds_list[j].start -= range_start;
                    seeds_list[j].stage += 1;
                } else if start_in_range && !end_in_range {
                    let cutoff_num = range_last - seed_start + 1;
                    seeds_list.push(ValPart2 {
                        start: range_last + 1,
                        len: seed.len - cutoff_num,
                        stage: seed.stage,
                    });
                    seeds_list[j].len = cutoff_num;
                    seeds_list[j].start += dest_start;
                    seeds_list[j].start -= range_start;
                    seeds_list[j].stage += 1;
                } else if !start_in_range && end_in_range {
                    seeds_list.push(ValPart2 {
                        start: seed_start,
                        len: range_start - seed_start,
                        stage: seed.stage,
                    });
                    seeds_list[j].len = seed_last - range_start + 1;
                    seeds_list[j].start = dest_start;
                    seeds_list[j].stage += 1;
                } else if seed_start < range_start
                    && seed_last > range_start
                    && seed_last > range_last
                    && seed_start < range_last
                {
                    seeds_list.push(ValPart2 {
                        start: seed_start,
                        len: range_start - seed_start,
                        stage: seed.stage,
                    });
                    let cutoff_num = range_last - seed_start + 1;
                    seeds_list.push(ValPart2 {
                        start: range_last + 1,
                        len: seeds_list[j].len - cutoff_num,
                        stage: seeds_list[j].stage,
                    });
                    seeds_list[j].len = range_last - range_start + 1;
                    seeds_list[j].start = dest_start;
                    seeds_list[j].stage += 1;
                }
                j += 1;
            }
        }
        for seed in seeds_list.iter_mut() {
            if seed.stage == i {
                seed.stage += 1;
            }
        }
    }
    seeds_list.iter().map(|x| x.start).min().unwrap()
}
fn part2(input: &str) -> usize {
    let mut seeds_list: Vec<ValPart2> = input
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>()
        .chunks(2)
        .map(|nums| ValPart2 {
            start: nums[0],
            len: nums[1],
            stage: 0,
        })
        .collect();
    get_min_location_2(input, &mut seeds_list)
}

#[cfg(test)]
#[test]
fn test_part2() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_eq!(part2(input), 46);
}
