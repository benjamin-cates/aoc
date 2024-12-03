fn main() {
    let input: &str = include_str!("../data/03.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

// Finished in 8:30
fn part1(mut input: &str) -> usize {
    let mut sum = 0usize;
    while let Some(mul) = input.find("mul(") {
        let bytes = input.as_bytes();
        let mut cursor = mul + 4;
        while bytes[cursor] >= b'0' && bytes[cursor] <= b'9' {
            cursor += 1;
        }
        let comma = cursor;
        if bytes[comma] != b',' {
            input = &input[cursor..];
            continue;
        }
        cursor += 1;
        while bytes[cursor] >= b'0' && bytes[cursor] <= b'9' {
            cursor += 1;
        }
        if bytes[cursor] != b')' {
            input = &input[cursor..];
            continue;
        }
        let num1 = input[mul + 4..comma].parse::<usize>().unwrap();
        let num2 = input[comma + 1..cursor].parse::<usize>().unwrap();
        sum += num1 * num2;
        input = &input[cursor + 1..];
    }
    sum
}

// Finished in 18:15
fn part2(mut input: &str) -> usize {
    let mut sum = 0usize;
    let mut enabled = true;
    while let Some(mul) = input.find("mul(") {
        let do_statement = input[0..mul + 1].rfind("do()");
        let dont_statement = input[0..mul + 1].rfind("don't()");
        enabled = match (do_statement, dont_statement) {
            (Some(dos), Some(donts)) => donts > dos,
            (Some(_dos), None) => true,
            (None, Some(_donts)) => false,
            (None, None) => enabled,
        };
        if !enabled {
            input = &input[mul + 4..];
            continue;
        }
        let bytes = input.as_bytes();
        let mut cursor = mul + 4;
        while bytes[cursor] >= b'0' && bytes[cursor] <= b'9' {
            cursor += 1;
        }
        let comma = cursor;
        if bytes[comma] != b',' {
            input = &input[cursor..];
            continue;
        }
        cursor += 1;
        while bytes[cursor] >= b'0' && bytes[cursor] <= b'9' {
            cursor += 1;
        }
        if bytes[cursor] != b')' {
            input = &input[cursor..];
            continue;
        }
        let num1 = input[mul + 4..comma].parse::<usize>().unwrap();
        let num2 = input[comma + 1..cursor].parse::<usize>().unwrap();
        sum += num1 * num2;
        input = &input[cursor + 1..];
    }
    sum
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(part1(input), 161);
    let input = "xmul(2,4)%&mul[3,7]!@^don't()_mul(5,5)+mul(32,64]then(mul(11,8)undo()?mul(8,5))";
    assert_eq!(part2(input), 48);
}
