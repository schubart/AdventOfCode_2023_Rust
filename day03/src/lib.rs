#![cfg(test)]

use std::collections::{HashMap, HashSet};

type Digits = HashMap<(isize, isize), char>;
type Symbols = HashMap<(isize, isize), char>;

fn solve1() -> usize {
    let (mut digits, symbols) = parse();

    symbols
        .keys()
        .flat_map(|&(x, y)| neighbors(x, y, &mut digits))
        .sum()
}

fn solve2() -> usize {
    let (mut digits, symbols) = parse();

    symbols
        .iter()
        .filter_map(|(point, &symbol)| (symbol == '*').then_some(point))
        .map(|&(x, y)| neighbors(x, y, &mut digits))
        .filter(|neighbors| neighbors.len() == 2)
        .map(|neighbors| neighbors.iter().product::<usize>())
        .sum()
}

fn parse() -> (Digits, Symbols) {
    let mut digits = HashMap::new();
    let mut symbols = HashMap::new();

    for (y, line) in include_str!("input.txt").lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x = isize::try_from(x).unwrap();
            let y = isize::try_from(y).unwrap();

            if c.is_ascii_digit() {
                digits.insert((x, y), c);
            } else if c != '.' {
                symbols.insert((x, y), c);
            }
        }
    }

    (digits, symbols)
}

fn neighbors(x: isize, y: isize, digits: &mut Digits) -> HashSet<usize> {
    #[rustfmt::skip]
    let deltas = [
        (-1, -1), (0, -1), (1,  -1),
        (-1,  0),          (1,   0),
        (-1,  1), (0,  1), (1,   1),
    ];

    deltas
        .iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter_map(|(x, y)| find(x, y, digits))
        .collect()
}

fn find(x: isize, y: isize, digits: &mut Digits) -> Option<usize> {
    let mut result = digits.get(&(x, y))?.to_string();

    // Search left.
    let mut start = x - 1;
    while let Some(d) = digits.remove(&(start, y)) {
        result.insert(0, d);
        start -= 1;
    }

    // Search right.
    let mut end = x + 1;
    while let Some(d) = digits.remove(&(end, y)) {
        result.push(d);
        end += 1;
    }

    Some(result.parse().unwrap())
}

#[test]
fn test_part1() {
    assert_eq!(532_331, solve1());
}

#[test]
fn test_part2() {
    assert_eq!(82_301_120, solve2());
}
