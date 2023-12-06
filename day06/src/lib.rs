#![cfg(test)]

fn solve(ignore_whitespace: bool) -> usize {
    let mut lines = include_str!("input.txt").lines();
    let times = parse(lines.next().unwrap(), ignore_whitespace);
    let distances = parse(lines.next().unwrap(), ignore_whitespace);

    times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| {
            (1..time)
                .filter(|charge| charge * (time - charge) > distance)
                .count()
        })
        .product()
}

fn parse(line: &str, ignore_whitespace: bool) -> Vec<i64> {
    let mut numbers = line.split(':').nth(1).unwrap().to_string();

    if ignore_whitespace {
        numbers = numbers.replace(' ', "");
    }

    numbers
        .split_whitespace()
        .map(|part| part.parse::<i64>().unwrap())
        .collect()
}

#[test]
fn test_part1() {
    assert_eq!(1_731_600, solve(false));
}

#[test]
fn test_part2() {
    assert_eq!(40_087_680, solve(true));
}
