#![cfg(test)]
#![allow(clippy::cast_possible_truncation)]

use std::collections::{HashMap, HashSet};

fn solve1() -> usize {
    include_str!("input.txt")
        .lines()
        .map(count_matches)
        .map(|n| match n {
            0 => 0,
            n => 2_usize.pow(n as u32 - 1),
        })
        .sum()
}

fn solve2() -> usize {
    // How many copies of each card.
    let mut copies = HashMap::new();

    include_str!("input.txt")
        .lines()
        .map(count_matches)
        .enumerate()
        .for_each(|(card, card_matches)| {
            // How many copies of this card? One if not visited before.
            let card_copies = *copies.entry(card).or_insert(1);

            // Visit each subsequent card, up to the number of matches on this card.
            for offset in 1..=card_matches {
                // Add the number of copies of this card to the number of copies of the
                // visited card, which is one if it was not visited before.
                *copies.entry(card + offset).or_insert(1) += card_copies;
            }
        });

    copies.values().sum()
}

fn count_matches(line: &str) -> usize {
    let mut parts = line.split_whitespace().skip(2); // Ignore "Card" and "123:"
    let winning: HashSet<_> = parts.by_ref().take_while(|&part| part != "|").collect();

    parts.filter(|number| winning.contains(number)).count()
}

#[test]
fn test_part1() {
    assert_eq!(15_268, solve1());
}

#[test]
fn test_part2() {
    assert_eq!(6_283_755, solve2());
}
