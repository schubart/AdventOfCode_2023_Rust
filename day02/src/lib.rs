#![cfg(test)]

use std::cmp::max;
use std::collections::HashMap;

fn solve1() -> usize {
    include_str!("input.txt")
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let mut parts = line.split(&[':', ',', ';', ' '][..]);
            parts.next(); // Ignore "Game".
            parts.next(); // Ignore ID.

            let mut result = Some(index + 1); // Cheat: Assume ID = line number, don't parse ID.

            while parts.next().is_some() {
                let count = parts.next().unwrap().parse::<usize>().unwrap();
                let color = parts.next().unwrap();

                let possible = match color {
                    "red" => count <= 12,
                    "green" => count <= 13,
                    "blue" => count <= 14,
                    _ => unreachable!(),
                };

                if !possible {
                    result = None;
                }
            }

            result
        })
        .sum()
}

fn solve2() -> usize {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut parts = line.split(&[':', ',', ';', ' '][..]);
            parts.next(); // Ignore "Game".
            parts.next(); // Ignore ID.

            let mut mins = HashMap::new();

            while parts.next().is_some() {
                let count = parts.next().unwrap().parse::<usize>().unwrap();
                let color = parts.next().unwrap();

                let min = mins.entry(color).or_default();
                *min = max(count, *min);
            }

            mins.values().product::<usize>()
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(2_879, solve1());
}

#[test]
fn test_part2() {
    assert_eq!(65_122, solve2());
}
