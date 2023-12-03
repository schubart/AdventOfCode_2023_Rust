#![cfg(test)]

use std::cmp::max;
use std::collections::HashMap;

fn solve1() -> usize {
    include_str!("input.txt")
        .lines()
        .map(max_counts)
        .filter_map(|(id, samples)| {
            let plausible = samples.iter().all(|(&color, &count)| match color {
                "red" => count <= 12,
                "green" => count <= 13,
                "blue" => count <= 14,
                _ => false,
            });

            plausible.then_some(id)
        })
        .sum()
}

fn solve2() -> usize {
    include_str!("input.txt")
        .lines()
        .map(max_counts)
        .map(|(_, samples)| samples.values().product::<usize>())
        .sum()
}

// Parse a line, return the game ID and the maximum count of each color seen in the game.
fn max_counts(line: &str) -> (usize, HashMap<&str, usize>) {
    let mut parts = line.split(&[':', ',', ';', ' '][..]);
    let id = parts.nth(1).unwrap().parse().unwrap();

    let mut samples = HashMap::new();

    while parts.next().is_some() {
        let count = parts.next().unwrap().parse::<usize>().unwrap();
        let color = parts.next().unwrap();

        let max_count = samples.entry(color).or_default();
        *max_count = max(count, *max_count);
    }

    (id, samples)
}

#[test]
fn test_part1() {
    assert_eq!(2_879, solve1());
}

#[test]
fn test_part2() {
    assert_eq!(65_122, solve2());
}
