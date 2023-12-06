#![cfg(test)]
#![allow(clippy::suboptimal_flops)]
#![allow(clippy::cast_possible_truncation)]

fn solve(ignore_whitespace: bool) -> f64 {
    let mut lines = include_str!("input.txt").lines();
    let times = parse(lines.next().unwrap(), ignore_whitespace);
    let distances = parse(lines.next().unwrap(), ignore_whitespace);

    times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| {
            // Quadratic formula for distance between smallest and biggest sufficient charge.
            // https://en.wikipedia.org/wiki/Quadratic_formula
            let sqrt = (time.powi(2) - (4.0 * distance)).sqrt();
            let min = (time - sqrt) / 2.0;
            let max = (time + sqrt) / 2.0;

            max.ceil() - min.floor() - 1.0
        })
        .product()
}

fn parse(line: &str, ignore_whitespace: bool) -> Vec<f64> {
    let mut numbers = line.split(':').nth(1).unwrap().to_string();

    if ignore_whitespace {
        numbers = numbers.replace(' ', "");
    }

    numbers
        .split_whitespace()
        .map(|part| part.parse::<f64>().unwrap())
        .collect()
}

#[test]
fn test_part1() {
    assert_eq!(1_731_600, solve(false) as i64);
}

#[test]
fn test_part2() {
    assert_eq!(40_087_680, solve(true) as i64);
}
