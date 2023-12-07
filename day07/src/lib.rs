#![cfg(test)]

use std::cmp::Ordering;
use std::collections::HashMap;

fn solve(use_joker: bool) -> usize {
    let mut hand_bids: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            (
                parts.next().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    hand_bids.sort_by(|(hand1, _), (hand2, _)| compare(hand1, hand2, use_joker));

    hand_bids
        .iter()
        .enumerate()
        .map(|(index, (_, bid))| (index + 1) * bid)
        .sum()
}

fn compare(hand1: &str, hand2: &str, use_joker: bool) -> Ordering {
    let hand_strength1 = hand_strength(hand1, use_joker);
    let hand_strength2 = hand_strength(hand2, use_joker);
    let card_strengths1 = hand1.chars().map(|card| card_strength(card, use_joker));
    let card_strengths2 = hand2.chars().map(|card| card_strength(card, use_joker));

    hand_strength1
        .cmp(&hand_strength2)
        .then_with(|| card_strengths1.cmp(card_strengths2))
}

fn hand_strength(hand: &str, use_joker: bool) -> usize {
    let mut label_counts: HashMap<char, usize> = HashMap::new();
    for card in hand.chars() {
        *label_counts.entry(card).or_insert(0) += 1;
    }

    if use_joker {
        // Count the jokers.
        let joker_count = label_counts.get(&'J').copied();
        // Get reference to the count of the most frequent, non-joker label.
        let other_count = label_counts
            .iter_mut()
            .filter(|(&card, _)| card != 'J')
            .map(|(_, count)| count)
            .max();

        // If there are some jokers and there is another label that can benefit from them,
        if let (Some(joker_count), Some(other_count)) = (joker_count, other_count) {
            // then turn the jokers into that label,
            *other_count += joker_count;
            // and remove the jokers.
            label_counts.remove(&'J');
        }
    }

    let mut counts: Vec<_> = label_counts.values().copied().collect();
    counts.sort_unstable();

    #[rustfmt::skip]
    let result = match *counts {
        [5]             => 6, // Five of a kind
        [1, 4]          => 5, // Four of a kind
        [2, 3]          => 4, // Full house
        [1, 1, 3]       => 3, // Three of a kind
        [1, 2, 2]       => 2, // Two pair
        [1, 1, 1, 2]    => 1, // One pair
        [1, 1, 1, 1, 1] => 0, // High card
        _ => unreachable!(),
    };

    result
}

fn card_strength(card: char, use_joker: bool) -> u32 {
    if use_joker && card == 'J' {
        return 0;
    }

    match &card {
        '2'..='9' => card.to_digit(10).unwrap(),
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

#[test]
fn test_part1() {
    assert_eq!(253_638_586, solve(false));
}

#[test]
fn test_part2() {
    assert_eq!(253_253_225, solve(true));
}
