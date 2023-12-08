#![cfg(test)]

use std::collections::HashMap;
use std::mem::swap;

fn solve(start_postfix: &str, end_postfix: &str) -> usize {
    let mut lines = include_str!("input.txt").lines();
    let directions = lines.next().unwrap();

    let mut map = HashMap::new();
    for line in lines.skip(1) {
        let mut parts = line.split(&[' ', '=', '(', ',', ')'][..]);
        let from = parts.next().unwrap();
        let left = parts.nth(3).unwrap();
        let right = parts.nth(1).unwrap();

        map.insert(from, (left, right));
    }

    map.keys()
        .filter(|node| node.ends_with(start_postfix))
        .fold(1, |result, mut current| {
            let mut directions = directions.chars().cycle().enumerate();

            // This is not a general solution for the problem statement.
            // It only works for the (friendly) kind of input provided by Advent of Code.
            let length = loop {
                let (index, direction) = directions.next().unwrap();

                current = match (direction, &map[current]) {
                    ('L', (left, _right)) => left,
                    ('R', (_left, right)) => right,
                    _ => unreachable!(),
                };

                if current.ends_with(end_postfix) {
                    break index + 1;
                }
            };

            lcm(result, length)
        })
}

fn lcm(a: usize, b: usize) -> usize {
    // https://en.wikipedia.org/wiki/Least_common_multiple#Using_the_greatest_common_divisor
    a * b / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    // https://en.wikipedia.org/wiki/Euclidean_algorithm#Implementations
    while b != 0 {
        a %= b;
        swap(&mut a, &mut b);
    }

    a
}

#[test]
fn test_part1() {
    assert_eq!(18_157, solve("AAA", "ZZZ"));
}

#[test]
fn test_part2() {
    assert_eq!(14_299_763_833_181, solve("A", "Z"));
}
